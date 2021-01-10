extern crate cairo;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use std::cell::*;
use std::rc::*;

mod image;

pub fn new_app_drawingarea() -> gtk::DrawingArea {
    let builder = gtk::DrawingAreaBuilder::new().width_request(300);
    let drawing_area = builder.build();

    // note:
    // キーイベントを取り出せない
    // textarea系なら出来るのかな？windowから取れるので、今はそうしておく
    // マウスは取り出せる
    drawing_area
        .add_events(gdk::EventMask::BUTTON_PRESS_MASK | gdk::EventMask::BUTTON_RELEASE_MASK);

    drawing_area.connect_button_press_event(|_, event| {
        println!(
            "btnpress: {} ({}, {})",
            event.get_button(),
            event.get_position().0,
            event.get_position().1
        );
        Inhibit(false)
    });
    drawing_area.connect_button_release_event(|_, event| {
        println!(
            "btnrelease: {} ({}, {})",
            event.get_button(),
            event.get_position().0,
            event.get_position().1
        );
        Inhibit(false)
    });

    //

    let mut initial_image = image::Image::new(400, 400);

    initial_image.with_surface(|surface| {
        let cr = cairo::Context::new(surface);
        cr.set_source_rgb(0., 1., 0.);
        cr.paint();
    });
    //
    // This is the channel for sending results from the worker thread to the main thread
    // For every received image, queue the corresponding part of the DrawingArea for redrawing
    let (worker_to_gui_tx, worker_to_gui_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let (to_worker_tx, to_worker_rx) = std::sync::mpsc::channel::<RefCell<image::Image>>();

    // animation thread
    // std::thread::spawn(glib::clone!(move || {
    std::thread::spawn(move || {
        let mut n = 0;
        for image in to_worker_rx.iter() {
            n = (n + 1) % 100;

            // Draw an arc with a weirdly calculated radius
            image.borrow_mut().with_surface(|surface| {
                let context = cairo::Context::new(surface);
                context.set_source_rgb(1.0, 1.0, 1.0);
                context.paint();
                context.set_source_rgb(0.0, 0.0, n as f64 / 100.0);
                context.rectangle(50.0, 50.0, 100.0, 100.0);
                context.stroke();
                surface.flush();
            });

            // Send the finished image back to the GUI thread
            let _ = worker_to_gui_tx.send(image);
            std::thread::sleep(std::time::Duration::from_millis(15));
        }
    });

    let buffer_image = Rc::new(RefCell::new(initial_image.clone()));

    drawing_area.connect_draw(
      glib::clone!(@weak buffer_image => @default-return Inhibit(false), move|_ /* widget */, context: &cairo::Context| {
          buffer_image.borrow_mut().with_surface(|surface| {
                context.set_source_surface(surface, 0.0, 0.0);
                context.paint();
                context.set_source_rgb(0.0, 0.0, 0.0);
            });

            Inhibit(false)
        }),
    );

    let _ = to_worker_tx.send(RefCell::new(initial_image));

    worker_to_gui_rx.attach(
        None,
        glib::clone!(@weak drawing_area => @default-return Continue(false), move |image| {
            // Swap the newly received image with the old stored one and send the old one back to
            // the worker thread
            let next_image = image;
            buffer_image.swap(&next_image);
            let _ = to_worker_tx.send(next_image);

            drawing_area.queue_draw_area(0, 0, 400, 400); // TODO:

            Continue(true)
        }),
    );
    drawing_area
}
