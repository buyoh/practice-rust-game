extern crate cairo;
extern crate gio;
extern crate gtk;

mod renderer;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

//

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(400, 400);
        window.set_resizable(false);

        window.add_events(gdk::EventMask::KEY_PRESS_MASK | gdk::EventMask::KEY_RELEASE_MASK);
        window.connect_key_press_event(|_, event| {
            println!("press: {}", event.get_keycode().take().unwrap());
            Inhibit(false)
        });
        window.connect_key_release_event(|_, event| {
            println!("release: {}", event.get_keycode().take().unwrap());
            Inhibit(false)
        });

        let drawing_area = renderer::new_app_drawingarea();
        drawing_area.emit_grab_focus();
        window.add(&drawing_area);
        window.show_all();
    });

    application.run(&[]);
}
