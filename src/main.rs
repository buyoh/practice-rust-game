extern crate cairo;
extern crate gio;
extern crate gtk;

mod game;
mod input;
mod renderer;
mod screen;

use std::{cell::RefCell, rc::Rc};

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

        let (input, driver_raw) = input::Input::create_with_driver();
        let game = std::sync::Arc::new(std::sync::Mutex::new(game::Game::new(input)));
        let input_driver = Rc::new(RefCell::new(driver_raw));

        // for transfering display infomations
        let (game_display_tx, game_display_rx) = std::sync::mpsc::channel::<game::GameRenderInfo>();

        window.add_events(gdk::EventMask::KEY_PRESS_MASK | gdk::EventMask::KEY_RELEASE_MASK);
        {
            let idr = input_driver.clone();
            window.connect_key_press_event(move |_, event| {
                idr.borrow_mut().handle_key_press_event(event.get_keyval());
                Inhibit(false)
            });
        }
        {
            let idr = input_driver.clone();
            window.connect_key_release_event(move |_, event| {
                idr.borrow_mut()
                    .handle_key_release_event(event.get_keyval());
                Inhibit(false)
            });
        }

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
                    game_display_tx.send(g.get_render_info()).ok();
                }
                // タイマーが使えたら良い
                std::thread::sleep(std::time::Duration::from_millis(15));
            }
        });
    });

    application.run(&[]);
}
