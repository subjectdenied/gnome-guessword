mod game;

use std::rc::Rc;
use std::cell::RefCell;
use gtk::prelude::*;
use game::*;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {

    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/com/subjectdenied/gnome-guessword/window.ui");
        let widget: gtk::ApplicationWindow = builder.get_object("window").expect("Failed to find the window object");
        let container_main: gtk::Box = builder.get_object("container_main").expect("Failed to find the window object");
        let button_new_game: gtk::Button = builder.get_object("button_new_game").expect("Failed to load button");

        // init game
        // use refcell to get a pointer
        let game = Rc::new(RefCell::new(Game::new(builder, 9)));

         // create reference to it
        let handle_start = game.clone();

        // borrow for starting the game
        game.borrow_mut().start(true);

        // key-press listener
        widget.connect_key_release_event(move |_, key| {
            if game.borrow().running {
                let key_val = key.get_keyval() as u8 as char;
                println!("current word on keypress: {}", game.borrow().word.current);
                // borrow mut is the key here to have
                // else we have the issue that closure functions cannot mutate moved vars
                let solved = game.borrow_mut().guess(key_val);
                game.borrow_mut().render_points();
                game.borrow_mut().render_board();

                if solved {
                    game.borrow_mut().start(false);
                }
            }

            Inhibit(false)
        });

        button_new_game.connect_clicked(move |_| {
            handle_start.borrow_mut().start(true);
        });

        widget.add(&container_main);

        widget.show_all();

        Self {
            widget
        }
    }

}

