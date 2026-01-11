use gtk::prelude::*;
use relm4::prelude::*;

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

#[relm4::component]
impl SimpleComponent for SignupScreen {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view!(gtk::CenterBox {});

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SignupScreen;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

struct LoginScreen;

#[relm4::component]
impl SimpleComponent for LoginScreen {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view!(gtk::CenterBox {});

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = LoginScreen;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

struct App {
    screen: Screens,
    main: Controller<MainScreen>,
    signup: Controller<SignupScreen>,
    login: Controller<LoginScreen>,
}

#[derive(Debug)]
enum AppMsg {
    Signup,
    Login,
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

        let model = App {
            screen: Screens::Main,
            main: main,
            login: login,
            signup: signup,
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
        }
    }
}

fn main() {
    let app = RelmApp::new("password.org");
    app.run::<App>(());
}
