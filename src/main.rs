extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, BoxBuilder, Button, DrawingAreaBuilder};

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 140);

        let box_builder = BoxBuilder::new();
        let linear = box_builder.build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        linear.add(&button);

        let builder = DrawingAreaBuilder::new().width_request(300);
        let drawing_area = builder.build();
        drawing_area.connect_draw(|_ /* widget */, context| {
            context.set_source_rgb(1.0, 1.0, 1.0);
            context.paint();
            context.set_source_rgb(0.0, 0.0, 0.0);
            context.rectangle(50.0, 50.0, 100.0, 100.0);
            context.stroke();
            return Inhibit(false);
        });
        linear.add(&drawing_area);

        window.add(&linear);
        window.show_all();
    });

    application.run(&[]);
}
