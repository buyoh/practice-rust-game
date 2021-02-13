extern crate cairo;
extern crate gio;
extern crate gtk;

mod game;
mod renderer;
mod screen;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

//

const DISPLAY_WIDTH: i32 = 400;
const DISPLAY_HEIGHT: i32 = 400;

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(DISPLAY_WIDTH, DISPLAY_HEIGHT);
        window.set_resizable(false);

        let game = std::sync::Arc::new(std::sync::Mutex::new(game::Game::new()));

        // for transfering display infomations
        let (game_display_tx, game_display_rx) =
            std::sync::mpsc::channel::<game::GameDisplayInfo>();

        // これじゃない感
        let game1 = game.clone();
        let game2 = game.clone();

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

        let renderer = renderer::Renderer::new(DISPLAY_WIDTH, DISPLAY_HEIGHT, game_display_rx);

        let drawing_area = screen::new_app_drawingarea(DISPLAY_WIDTH, DISPLAY_HEIGHT, renderer);
        drawing_area.emit_grab_focus();
        window.add(&drawing_area);
        window.show_all();

        // game main loop
        std::thread::spawn(move || {
            let tt = std::time::Instant::now();
            loop {
                {
                    let mut g = game.lock().unwrap();
                    g.tick(tt.elapsed().as_secs_f64());
                    game_display_tx.send(g.get_display_info()).ok();
                }
                // タイマーが使えたら良い
                std::thread::sleep(std::time::Duration::from_millis(15));
            }
        });
    });

    application.run(&[]);
}
