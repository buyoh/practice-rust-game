extern crate cairo;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use std::cell::*;
use std::rc::*;

use crate::renderer::{Renderer, RendererHolder}; // note: いまいち？

mod image;

pub fn new_app_drawingarea(width: i32, height: i32, renderer: Renderer) -> gtk::DrawingArea {
    let builder = gtk::DrawingAreaBuilder::new().width_request(300);
    let drawing_area = builder.build();

    // note:
    // キーイベントを取り出せない
    // textarea系なら出来るのかな？windowから取れるので、今はそうしておく
    // マウスは取り出せる
    drawing_area
        .add_events(gdk::EventMask::BUTTON_PRESS_MASK | gdk::EventMask::BUTTON_RELEASE_MASK);

    drawing_area.connect_button_press_event(|_, _event| {
        // TODO: handle mouse event
        // println!(
        //     "btnpress: {} ({}, {})",
        //     event.get_button(),
        //     event.get_position().0,
        //     event.get_position().1
        // );
        Inhibit(false)
    });
    drawing_area.connect_button_release_event(|_, _event| {
        // TODO: handle mouse event
        // println!(
        //     "btnrelease: {} ({}, {})",
        //     event.get_button(),
        //     event.get_position().0,
        //     event.get_position().1
        // );
        Inhibit(false)
    });

    //

    let mut initial_image = image::Image::new(width, height);

    initial_image.with_surface(|surface| {
        let cr = cairo::Context::new(surface);
        cr.set_source_rgb(0., 1., 0.);
        cr.paint();
    });
    //
    // This is the channel for sending results from the worker thread to the main thread
    // For every received image, queue the corresponding part of the DrawingArea for redrawing
    // gtkのメインスレッドで実行したい場合に使えるchannel
    // 受け取った時に実行するには、worker_to_gui_rx.attachを実装する
    let (worker_to_gui_tx, worker_to_gui_rx) =
        glib::MainContext::channel::<Box<image::Image>>(glib::PRIORITY_DEFAULT);

    // from main thread to worker thread
    let (to_worker_tx, to_worker_rx) = std::sync::mpsc::channel::<Box<image::Image>>();

    // animation thread
    std::thread::spawn(move || {
        // Hold renderer here
        let mut rr = RendererHolder::new(renderer);
        for mut image in to_worker_rx.iter() {
            image.with_surface(|surface| {
                let context = cairo::Context::new(surface);
                rr.paint_game(&context);
                surface.flush();
            });

            // Send the finished image back to the GUI thread
            let _ = worker_to_gui_tx.send(image);
            std::thread::sleep(std::time::Duration::from_millis(15));
        }
    });

    let buffer_image = Rc::new(RefCell::new(Box::new(initial_image.clone()))); // これは頭悪い？
    let _ = to_worker_tx.send(Box::new(initial_image));

    drawing_area.connect_draw(
      glib::clone!(@weak buffer_image => @default-return Inhibit(false), move|_ /* widget */, context: &cairo::Context| {
            // 描画が必要になった時に呼び出される
            // buffer_image が死んだ時は呼び出されず Inhibit(false) を返す
            let mut img = buffer_image.borrow_mut();
            img.with_surface(|surface| {
                context.set_source_surface(surface, 0.0, 0.0);
                context.paint();
                context.set_source_rgb(0.0, 0.0, 0.0);
            });
            Inhibit(false)
        }),
    );

    worker_to_gui_rx.attach(
        None,
        glib::clone!(@weak drawing_area => @default-return Continue(false), move |image| {
            // Swap the newly received image with the old stored one and send the old one back to
            // the worker thread
            let width = image.width();
            let height = image.height();
            let img = buffer_image.replace(image);
            let _ = to_worker_tx.send(img);

            // Query redrawing
            drawing_area.queue_draw_area(0, 0, width, height);

            Continue(true)
        }),
    );
    drawing_area
}
