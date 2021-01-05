extern crate cairo;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use std::cell::*;
use std::rc::*;

use gtk::{Application, ApplicationWindow, BoxBuilder, Button, DrawingAreaBuilder};

//

const WIDTH: i32 = 400;
const HEIGHT: i32 = 400;

// Our custom image type. This stores a heap allocated byte array for the pixels for each of our
// images, can be sent safely between threads and can be temporarily converted to a Cairo image
// surface for drawing operations
#[derive(Clone)]
struct Image(Option<Box<[u8]>>);

impl Image {
    // Creates a new, black image
    fn new() -> Self {
        Image(Some(vec![0; 4 * WIDTH as usize * HEIGHT as usize].into()))
    }

    // Calls the given closure with a temporary Cairo image surface. After the closure has returned
    // there must be no further references to the surface.
    fn with_surface<F: FnOnce(&cairo::ImageSurface)>(&mut self, func: F) {
        // Helper struct that allows passing the pixels to the Cairo image surface and once the
        // image surface is destroyed the pixels will be stored in the return_location.
        //
        // This allows us to give temporary ownership of the pixels to the Cairo surface and later
        // retrieve them back in a safe way while ensuring that nothing else still has access to
        // it.
        struct ImageHolder {
            image: Option<Box<[u8]>>,
            return_location: Rc<RefCell<Option<Box<[u8]>>>>,
        }

        // This stores the pixels back into the return_location as now nothing
        // references the pixels anymore
        impl Drop for ImageHolder {
            fn drop(&mut self) {
                *self.return_location.borrow_mut() =
                    Some(self.image.take().expect("Holding no image"));
            }
        }

        // Needed for ImageSurface::create_for_data() to be able to access the pixels
        impl AsRef<[u8]> for ImageHolder {
            fn as_ref(&self) -> &[u8] {
                self.image.as_ref().expect("Holding no image").as_ref()
            }
        }

        impl AsMut<[u8]> for ImageHolder {
            fn as_mut(&mut self) -> &mut [u8] {
                self.image.as_mut().expect("Holding no image").as_mut()
            }
        }

        // Temporary move out the pixels
        let image = self.0.take().expect("Empty image");

        // A new return location that is then passed to our helper struct below
        let return_location = Rc::new(RefCell::new(None));
        {
            let holder = ImageHolder {
                image: Some(image),
                return_location: return_location.clone(),
            };

            // The surface will own the image for the scope of the block below
            let surface = cairo::ImageSurface::create_for_data(
                holder,
                cairo::Format::Rgb24,
                WIDTH,
                HEIGHT,
                4 * WIDTH,
            )
            .expect("Can't create surface");
            func(&surface);

            // Now the surface will be destroyed and the pixels are stored in the return_location
        }

        // And here move the pixels back again
        self.0 = Some(
            return_location
                .borrow_mut()
                .take()
                .expect("Image not returned"),
        );
    }
}

//

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

        linear.add(&drawing_area);

        window.add(&linear);
        window.show_all();

        //
        let mut initial_image = Image::new();

        initial_image.with_surface(|surface| {
            let cr = cairo::Context::new(surface);
            cr.set_source_rgb(0., 1., 0.);
            cr.paint();
        });
        //
        // This is the channel for sending results from the worker thread to the main thread
        // For every received image, queue the corresponding part of the DrawingArea for redrawing
        let (worker_to_gui_tx, worker_to_gui_rx) =
            glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let (to_worker_tx, to_worker_rx) = std::sync::mpsc::channel::<RefCell<Image>>();

        // let mut tick1 = move || {
        // };
        // gtk::timeout_add_seconds(1, tick1);

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
                    context.set_source_rgb(0.0, 0.0, n as f64/100.0);
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

        worker_to_gui_rx.attach(None, move |image| {
            // Swap the newly received image with the old stored one and send the old one back to
            // the worker thread
            let next_image = image;
            buffer_image.swap(&next_image);
            let _ = to_worker_tx.send(next_image);

            drawing_area.queue_draw_area(0, 0, WIDTH, HEIGHT);

            Continue(true)
        });
    });

    application.run(&[]);
}
