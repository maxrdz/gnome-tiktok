/* feed_carousel.rs
 *
 * Copyright 2024 GNOME TuxTok Authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::gtk;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use libadwaita as adw;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/GnomeTuxTok/ui/feed-carousel.ui")]
    pub struct FeedCarousel {
        #[template_child]
        pub video_carousel: TemplateChild<adw::Carousel>,
        #[template_child]
        pub video_widget_1: TemplateChild<gtk::Video>,
        #[template_child]
        pub video_widget_2: TemplateChild<gtk::Video>,
        #[template_child]
        pub video_widget_3: TemplateChild<gtk::Video>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FeedCarousel {
        const NAME: &'static str = "FeedCarousel";
        type Type = super::FeedCarousel;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FeedCarousel {
        fn constructed(&self) {
            self.parent_constructed();
            let _obj = self.obj(); // our class' widget, inhereting from AdwBin
        }
    }
    impl WidgetImpl for FeedCarousel {}
    impl BinImpl for FeedCarousel {}
}

glib::wrapper! {
    pub struct FeedCarousel(ObjectSubclass<imp::FeedCarousel>)
        @extends gtk::Widget, adw::Bin,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl FeedCarousel {
    pub fn new<P: glib::IsA<adw::gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
