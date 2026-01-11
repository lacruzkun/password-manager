use std::convert::identity;

use gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
enum Screens {
    Main,
    Signup,
    AccountCreated,
    Login,
    RetrievePassword,
    AddPassword,
    Password,
    Welcome,
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

struct SignupScreen;

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

                    },
                },

                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        set_label: "Password"
                    },

                    gtk::Entry {

                    },
                },

                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        set_label: "Confirm password"
                    },

                    gtk::Entry {

                    },
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
        let model = SignupScreen;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SignLogMsg::Ok => {
                let _ = sender.output(AppMsg::Modal(Screens::Signup));
            }
        }
    }
}

struct LoginScreen;

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

                    },
                },

                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        set_label: "Password"
                    },

                    gtk::Entry {

                    },
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
        let model = LoginScreen;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SignLogMsg::Ok => {
                let _ = sender.output(AppMsg::Modal(Screens::Login));
            }
        }
    }
}

struct WelcomeDialog {
    hidden: bool,
}

#[derive(Debug)]
enum WelMsg {
    Show,
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
        set_text: Some("Welcome Back"),

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
        let model = WelcomeDialog { hidden: true };

        let widgets = view_output!();
        ComponentParts { widgets, model }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            WelMsg::Close => {
                self.hidden = true;
            }

            WelMsg::Show => {
                self.hidden = false;
            }
        }
    }
}

struct App {
    screen: Screens,
    welcome: Controller<WelcomeDialog>,
    main: Controller<MainScreen>,
    signup: Controller<SignupScreen>,
    login: Controller<LoginScreen>,
}

#[derive(Debug)]
enum AppMsg {
    Signup,
    Login,
    Modal(Screens),
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

            #[watch]
            set_visible_child_name: match model.screen {
                Screens::Main => "Main",
                Screens::Signup =>"Signup" ,
                Screens::AccountCreated =>"AccountCreated" ,
                Screens::Login =>"Login" ,
                Screens::RetrievePassword =>"RetrievePassword" ,
                Screens::AddPassword =>"AddPassword" ,
                Screens::Password =>"Password" ,
                Screens::Welcome =>"Welcome" ,
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

        let model = App {
            screen: Screens::Main,
            main: main,
            login: login,
            signup: signup,
            welcome: welcome,
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

            AppMsg::Modal(screen) => match screen {
                Screens::Login => {
                    self.welcome.emit(WelMsg::Show);
                }
                Screens::Signup => {
                    self.screen = Screens::AccountCreated;
                }
                _ => (),
            },
        }
    }
}

fn main() {
    let app = RelmApp::new("password.org");
    app.run::<App>(());
}
