use std::convert::identity;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufRead, BufReader, Write};

use gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Screens {
    Main,
    Signup,
    AccountCreated,
    Login,
    RetrievePassword,
    AddPassword,
    Password,
    UserSession,
}

struct MainScreen;

#[derive(Debug)]
enum MainMsg {
    Signup,
    Login,
    Quit,
}

#[relm4::component]
impl SimpleComponent for MainScreen {
    type Input = MainMsg;
    type Output = AppMsg;
    type Init = ();

    view!(gtk::CenterBox {
        set_orientation: gtk::Orientation::Horizontal,

        #[wrap(Some)]
        set_center_widget = &gtk::CenterBox{
            set_orientation: gtk::Orientation::Vertical,

            #[wrap(Some)]
            set_center_widget = &gtk::Box{
                set_spacing: 10,

                gtk::Button{
                    set_label: "Signup",
                    connect_clicked => MainMsg::Signup,
                },

                gtk::Button{
                    set_label: "Login",
                    connect_clicked => MainMsg::Login,
                },

                gtk::Button{
                    set_label: "Quit",
                    connect_clicked => MainMsg::Quit,
                },
            },
        },
    });

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = MainScreen {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            MainMsg::Signup => {
                let _ = sender.output(AppMsg::Signup);
            }

            MainMsg::Login => {
                let _ = sender.output(AppMsg::Login);
            }

            MainMsg::Quit => {
                let _ = sender.output(AppMsg::Quit);
            }
        };
    }
}

struct SignupScreen {
    username: gtk::EntryBuffer,
    password: gtk::PasswordEntry,
    password2: gtk::PasswordEntry,
}

#[derive(Debug)]
enum SignLogMsg {
    Ok,
}

#[relm4::component]
impl SimpleComponent for SignupScreen {
    type Init = ();
    type Input = SignLogMsg;
    type Output = AppMsg;

    view!(gtk::CenterBox {
        #[wrap(Some)]
        set_center_widget =  &gtk::CenterBox {
            set_orientation: gtk::Orientation::Vertical,

            #[wrap(Some)]
            set_center_widget = &gtk::Box{
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 20,

                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        set_label: "UserName"
                    },

                    gtk::Entry {
                        set_buffer: &model.username,
                    },
                },

                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        set_label: "Password"
                    },

                    append: &model.password,
                },

                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        set_label: "Confirm password"
                    },

                    append: &model.password2,
                },
            },
        },

        #[wrap(Some)]
        set_end_widget = &gtk::CenterBox{
            set_margin_all: 50,
            set_orientation: gtk::Orientation::Vertical,

            #[wrap(Some)]
            set_end_widget = &gtk::Button{
                set_label: "OK",
                connect_clicked => SignLogMsg::Ok,
            },
        },
    });

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SignupScreen {
            username: gtk::EntryBuffer::default(),
            password: gtk::PasswordEntry::new(),
            password2: gtk::PasswordEntry::new(),
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SignLogMsg::Ok => {
                match signup(
                    self.username.text().to_string(),
                    &mut self.password.text().to_string(),
                ) {
                    Ok(_) => {
                        let _ = sender.output(AppMsg::Modal(Screens::Signup, None));
                    }
                    Err(e) => {
                        let _ = sender.output(AppMsg::Modal(Screens::Signup, Some(e.to_string())));
                    }
                };
            }
        }
    }
}

struct LoginScreen {
    username: gtk::EntryBuffer,
    password: gtk::PasswordEntry,
}

#[relm4::component]
impl SimpleComponent for LoginScreen {
    type Init = ();
    type Input = SignLogMsg;
    type Output = AppMsg;

    view!(gtk::CenterBox {
        #[wrap(Some)]
        set_center_widget =  &gtk::CenterBox {
            set_orientation: gtk::Orientation::Vertical,

            #[wrap(Some)]
            set_center_widget = &gtk::Box{
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 20,

                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        set_label: "User name"
                    },

                    gtk::Entry {
                        set_buffer: &model.username,
                    },
                },

                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        set_label: "Password"
                    },

                    append: &model.password,
                },
            },
        },

        #[wrap(Some)]
        set_end_widget = &gtk::CenterBox{
            set_margin_all: 50,
            set_orientation: gtk::Orientation::Vertical,

            #[wrap(Some)]
            set_end_widget = &gtk::Button{
                set_label: "OK",
                connect_clicked => SignLogMsg::Ok,
            },
        },
    });

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = LoginScreen {
            username: gtk::EntryBuffer::default(),
            password: gtk::PasswordEntry::new(),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SignLogMsg::Ok => {
                match login(
                    &self.username.text().as_str().to_string(),
                    &mut self.password.text().as_str().to_string(),
                ) {
                    Ok(_) => {
                        let _ = sender.output(AppMsg::Modal(Screens::Login, None));
                    }
                    Err(e) => {
                        let _ = sender.output(AppMsg::Modal(Screens::Login, Some(e.to_string())));
                    }
                }
            }
        }
    }
}

struct WelcomeDialog {
    hidden: bool,
    screen: Screens,
    error: Option<String>,
}

#[derive(Debug)]
enum WelMsg {
    Show(Screens, Option<String>),
    Close,
}

#[relm4::component]
impl SimpleComponent for WelcomeDialog {
    type Init = gtk::Window;
    type Output = AppMsg;
    type Input = WelMsg;

    view!(
        dialog = gtk::MessageDialog {
        set_transient_for: Some(&init),
        set_modal: true,
        #[watch]
        set_text: if let Some(e) = &model.error {
            Some(e)
        }
        else if model.screen == Screens::Login {
            Some("Welcome Back")
        }
        else{
            Some("Account Created")
        },

        add_button: ("Ok", gtk::ResponseType::Accept),

        #[watch]
        set_visible: !model.hidden,

        connect_response[sender] => move |_, _| {
            sender.input(WelMsg::Close)
        }
    });

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = WelcomeDialog {
            hidden: true,
            screen: Screens::Login,
            error: None,
        };

        let widgets = view_output!();
        ComponentParts { widgets, model }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            WelMsg::Close => {
                self.hidden = true;
                if let Some(_e) = self.error.clone() {
                } else if self.screen == Screens::Login {
                    let _ = sender.output(AppMsg::User);
                } else if self.screen == Screens::Signup {
                    let _ = sender.output(AppMsg::Main);
                }
            }

            WelMsg::Show(screen, error) => {
                self.screen = screen;
                self.error = error;
                self.hidden = false;
            }
        }
    }
}

struct UserSessionScreen;

#[derive(Debug)]
enum UserMsg {
    Add,
    Retrieve,
}

#[relm4::component]
impl SimpleComponent for UserSessionScreen {
    type Input = UserMsg;
    type Output = AppMsg;
    type Init = ();

    view!(gtk::CenterBox {
        #[wrap(Some)]
        set_center_widget = &gtk::CenterBox {
            set_orientation: gtk::Orientation::Vertical,

            #[wrap(Some)]
            set_center_widget = &gtk::Box{
                set_spacing: 20,

                gtk::Button {
                    set_label: "Add Password",
                    connect_clicked => UserMsg::Add,
                },

                gtk::Button {
                    set_label: "Retrieve Password",
                    connect_clicked => UserMsg::Retrieve,
                },
            },
        },
    });

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = UserSessionScreen;

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            UserMsg::Add => {
                let _ = sender.output(AppMsg::AddPass);
            }

            UserMsg::Retrieve => {
                let _ = sender.output(AppMsg::RetrievePass);
            }
        }
    }
}

struct AddScreen;

#[relm4::component]
impl SimpleComponent for AddScreen {
    type Input = ();
    type Output = AppMsg;
    type Init = ();

    view!(gtk::Label {
        set_label: "Add Screen",
    });

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AddScreen;

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

struct RetrieveScreen;

#[relm4::component]
impl SimpleComponent for RetrieveScreen {
    type Input = ();
    type Output = AppMsg;
    type Init = ();

    view!(gtk::Label {
        set_label: "Add Screen",
    });

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = RetrieveScreen;

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

struct App {
    screen: Screens,
    welcome: Controller<WelcomeDialog>,
    main: Controller<MainScreen>,
    signup: Controller<SignupScreen>,
    login: Controller<LoginScreen>,
    usersession: Controller<UserSessionScreen>,
    add: Controller<AddScreen>,
    retrieve: Controller<RetrieveScreen>,
}

#[derive(Debug)]
enum AppMsg {
    Signup,
    Login,
    Main,
    Modal(Screens, Option<String>),
    User,
    AddPass,
    RetrievePass,
    Quit,
}

#[relm4::component]
impl SimpleComponent for App {
    type Input = AppMsg;
    type Output = ();
    type Init = ();

    view!(gtk::Window {
        set_title: Some("Password Manager"),

        gtk::Stack {
            set_transition_type: gtk::StackTransitionType::SlideLeftRight,
            set_transition_duration: 250,
            add_named: (model.main.widget(), Some("Main")),
            add_named: (model.signup.widget(), Some("Signup")),
            add_named: (model.login.widget(), Some("Login")),
            add_named: (model.usersession.widget(), Some("UserSession")),
            add_named: (model.add.widget(), Some("AddPassword")),
            add_named: (model.retrieve.widget(), Some("RetrievePassword")),

            #[watch]
            set_visible_child_name: match model.screen {
                Screens::Main => "Main",
                Screens::Signup =>"Signup" ,
                Screens::AccountCreated =>"AccountCreated" ,
                Screens::Login =>"Login" ,
                Screens::RetrievePassword =>"RetrievePassword" ,
                Screens::AddPassword =>"AddPassword" ,
                Screens::Password =>"Password" ,
                Screens::UserSession =>"UserSession" ,
            },

        }
    });

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let main = MainScreen::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);

        let login = LoginScreen::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);

        let signup = SignupScreen::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);

        let welcome = WelcomeDialog::builder()
            .launch(root.clone().upcast())
            .forward(sender.input_sender(), identity);

        let usersession = UserSessionScreen::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);

        let add = AddScreen::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);

        let retrieve = RetrieveScreen::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);

        let model = App {
            screen: Screens::Main,
            main: main,
            login: login,
            signup: signup,
            welcome: welcome,
            usersession: usersession,
            add: add,
            retrieve: retrieve,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Signup => {
                self.screen = Screens::Signup;
            }

            AppMsg::Login => {
                self.screen = Screens::Login;
            }

            AppMsg::Quit => {}

            AppMsg::Modal(modal, error) => match modal {
                Screens::Login => {
                    self.welcome.emit(WelMsg::Show(Screens::Login, error));
                }
                Screens::Signup => {
                    self.welcome.emit(WelMsg::Show(Screens::Signup, error));
                }
                _ => (),
            },

            AppMsg::User => {
                self.screen = Screens::UserSession;
            }

            AppMsg::Main => {
                self.screen = Screens::Main;
            }

            AppMsg::AddPass => {
                self.screen = Screens::AddPassword;
            }

            AppMsg::RetrievePass => {
                self.screen = Screens::RetrievePassword;
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("password.org");
    app.run::<App>(());
}

fn login<'a>(username: &String, password: &mut String) -> Result<(String, String), Box<dyn Error>> {
    let mut key = String::new();

    get_symmetric_key(&mut key, password);
    let mut password_hasher = DefaultHasher::new();
    password.hash(&mut password_hasher);
    *password = password_hasher.finish().to_string();

    let database = "master.db";
    let file = File::open(database)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let u: Vec<_> = line.split(" ").collect();
        if username == u[0] && password == u[1] {
            return Ok((username.clone(), key));
        }
    }

    Err("Can't find your account".into())
}

fn get_symmetric_key(key: &mut String, password: &String) {
    let mut hash: u128 = 5381;
    let password = password.clone().into_bytes().into_iter();
    for i in password {
        hash = hash.wrapping_mul(33).wrapping_add(i.into());
    }
    *key = hash.to_string();
}

fn signup(username: String, password: &mut String) -> Result<String, Box<dyn Error>> {
    let mut key = String::new();

    let mut password_hasher = DefaultHasher::new();

    get_symmetric_key(&mut key, &password);
    password.hash(&mut password_hasher);
    *password = password_hasher.finish().to_string();
    println!("\nMaster account {username} created");

    let user = username + " " + password.as_str() + "\n";

    let database = "master.db";

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(database)?;
    file.write_all(user.as_bytes())?;
    Ok(key)
}
