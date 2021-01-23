use std::cell::*;
use std::rc::*;

// Our custom image type. This stores a heap allocated byte array for the pixels for each of our
// images, can be sent safely between threads and can be temporarily converted to a Cairo image
// surface for drawing operations
#[derive(Clone)]
pub struct Image {
    width: i32,
    height: i32,
    data: Option<Box<[u8]>>,
}

impl Image {
    // Creates a new, black image
    pub fn new(width: i32, height: i32) -> Self {
        Image {
            width: width,
            height: height,
            data: Some(vec![0; 4 * width as usize * height as usize].into()),
        }
    }

    pub fn width(&self) -> i32 {
        self.width.clone()
    }

    pub fn height(&self) -> i32 {
        self.height.clone()
    }

    // Calls the given closure with a temporary Cairo image surface. After the closure has returned
    // there must be no further references to the surface.
    pub fn with_surface<F: FnOnce(&cairo::ImageSurface)>(&mut self, func: F) {
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
        let image = self.data.take().expect("Empty image");

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
                self.width,
                self.height,
                4 * self.width,
            )
            .expect("Can't create surface");
            func(&surface);

            // Now the surface will be destroyed and the pixels are stored in the return_location
        }

        // And here move the pixels back again
        self.data = Some(
            return_location
                .borrow_mut()
                .take()
                .expect("Image not returned"),
        );
    }
}
