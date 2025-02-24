// Copyright 2021 Rafael Mardojai CM
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::backend::{CardSize, Image};
use gtk::subclass::prelude::*;
use gtk::{self, prelude::*};
use gtk::{glib, CompositeTemplate};
use gtk_macros::spawn;

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/com/rafaelmardojai/SharePreview/image.ui")]
    pub struct CardImage {
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub image: TemplateChild<gtk::Picture>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CardImage {
        const NAME: &'static str = "CardImage";
        type Type = super::CardImage;
        type ParentType = gtk::Box;

        fn new() -> Self {
            Self {
                stack: TemplateChild::default(),
                spinner: TemplateChild::default(),
                image: TemplateChild::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CardImage {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for CardImage {}
    impl BoxImpl for CardImage {}
}

glib::wrapper! {
    pub struct CardImage(ObjectSubclass<imp::CardImage>)
        @extends gtk::Widget, gtk::Box;
}

impl CardImage {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create CardImage")
    }

    pub fn set_image(&self, img: &Image, size: &CardSize) {
        let imp = imp::CardImage::from_instance(self);

        // Get Widgets
        let stack = imp.stack.clone();
        let spinner = imp.spinner.clone();
        let image = imp.image.clone();

        let (width, height) = size.image_size(); // Get image size

        // Set image widget size
        image.set_width_request(width as i32);
        image.set_height_request(height as i32);

        spinner.start();

        // Fetch image and set it to the widget 
        let img = img.clone();
        spawn!(async move {
            match img.fetch(width, height).await {
                Ok(pixbuf) => {
                    image.set_pixbuf(Some(&pixbuf));
                    stack.set_visible_child_name("image");
                }
                Err(_) => {
                    spinner.stop();
                }
            }
        });
    }
}
