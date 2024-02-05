/* video_carousel.rs
 *
 * Copyright 2024 Unknown
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
    #[template(resource = "/com/maxrdz/GnomeTikTok/ui/video-carousel.ui")]
    pub struct VideoCarousel {}

    #[glib::object_subclass]
    impl ObjectSubclass for VideoCarousel {
        const NAME: &'static str = "VideoCarousel";
        type Type = super::VideoCarousel;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for VideoCarousel {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
    impl WidgetImpl for VideoCarousel {}
    impl BinImpl for VideoCarousel {}
}

glib::wrapper! {
    pub struct VideoCarousel(ObjectSubclass<imp::VideoCarousel>)
        @extends gtk::Widget, adw::Bin,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl VideoCarousel {
    pub fn new<P: glib::IsA<adw::gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
