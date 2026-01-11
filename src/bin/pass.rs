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

#[relm4::component]
impl SimpleComponent for MainScreen {
    type Input = ();
    type Output = ();
    type Init = ();

    view!(gtk::CenterBox {
        set_orientation: gtk::Orientation::Horizontal,

        #[wrap(Some)]
        set_center_widget = &gtk::CenterBox{
            set_orientation: gtk::Orientation::Vertical,

            #[wrap(Some)]
            set_center_widget = &gtk::Box{
                gtk::Button{
                    set_label: "Signup",
                },

                gtk::Button{
                    set_label: "Login",
                },

                gtk::Button{
                    set_label: "Quit",
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
}

struct App {
    screen: Screens,
    main: Controller<MainScreen>,
}

#[relm4::component]
impl SimpleComponent for App {
    type Input = ();
    type Output = ();
    type Init = ();

    view!(gtk::Window {
        set_title: Some("Password Manager"),

        gtk::Stack {
            set_transition_type: gtk::StackTransitionType::SlideLeftRight,
            set_transition_duration: 250,
            add_named: (model.main.widget(), Some("main")),

            #[watch]
            set_visible_child_name: match model.screen {
                Screens::Main => "main",
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
        let model = App {
            screen: Screens::Main,
            main: main,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("password.org");
    app.run::<App>(());
}
