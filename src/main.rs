use gtk::prelude::*;
use gio::prelude::*;
use gettextrs::*;
use std::env;

mod config;
mod window;
use crate::window::Window;

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("gnome-guessword", config::LOCALEDIR);
    textdomain("gnome-guessword");

    let res = gio::Resource::load(config::PKGDATADIR.to_owned() + "/gnome-guessword.gresource")
                                .expect("Could not load resources");
    gio::resources_register(&res);

    // css
    const STYLE: &str = "
        #fails {
            color: red;
            font-size: 16px;
        }
        #points {
            color: green;
            font-size: 16px;
        }
        #container_word label {
            border-left: 1px solid black;
            border-top: 1px solid black;
            border-right: 1px solid black;
            border-bottom: 1px solid black;
            background-color: white;
            font-size: 30px;
            font-family: Courier New;
        }
    ";

    let app = gtk::Application::new(Some("com.subjectdenied.gnome-guessword"), Default::default()).unwrap();
    app.connect_activate(move |app| {
        let window = Window::new();

        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(STYLE.as_bytes())
            .expect("Failed to load CSS");
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        window.widget.set_application(Some(app));
        app.add_window(&window.widget);
        window.widget.present();
    });


    let args: Vec<String> = env::args().collect();
    app.run(&args);

}

