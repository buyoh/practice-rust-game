extern crate cairo;
extern crate gio;
extern crate gtk;

mod game;
mod renderer;

use std::{cell::RefCell, rc::Rc};

use game::GameDisplayInfo;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

//

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    // let game = std::sync::Arc::new(game::Game::new());
    let game = std::sync::Arc::new(std::sync::Mutex::new(game::Game::new()));
    let game_gtk = game.clone();

    // for transfering display infomations
    let (game_display_tx, game_display_rx) = std::sync::mpsc::channel::<game::GameDisplayInfo>();
    // std::sync::mpsc::channel::<RefCell<game::GameDisplayInfo>>();

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(400, 400);
        window.set_resizable(false);

        // これじゃない感
        let game1 = game_gtk.clone();
        let game2 = game_gtk.clone();

        window.add_events(gdk::EventMask::KEY_PRESS_MASK | gdk::EventMask::KEY_RELEASE_MASK);
        window.connect_key_press_event(move |_, event| {
            match game1.lock() {
                Ok(mut g) => g.handle_key_press_event(event.get_keyval()),
                Err(_) => (),
            }
            println!("press: {}", event.get_keycode().take().unwrap());
            Inhibit(false)
        });
        window.connect_key_release_event(move |_, event| {
            match game2.lock() {
                Ok(mut g) => g.handle_key_release_event(event.get_keyval()),
                Err(_) => (),
            }
            println!("release: {}", event.get_keycode().take().unwrap());
            Inhibit(false)
        });

        let drawing_area = renderer::new_app_drawingarea(|| {
            let game_display = Rc::new(RefCell::new(GameDisplayInfo::default()));
            || {
                for new_game_display in game_display_rx.try_iter() {
                    game_display.replace(new_game_display);
                }
                game_display.borrow()
            }
        }());
        drawing_area.emit_grab_focus();
        window.add(&drawing_area);
        window.show_all();
    });

    std::thread::spawn(move || {
        let mut n = 0;
        loop {
            n = n + 1;
            {
                let mut g = game.lock().unwrap();
                g.tick();
                // game_display_tx.send(RefCell::new(g.get_display_info()));
                game_display_tx.send(g.get_display_info());
            }
            // タイマーが使えたら良い
            std::thread::sleep(std::time::Duration::from_millis(15));
        }
    });

    application.run(&[]);
}
