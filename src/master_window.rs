/* master_window.rs
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

use crate::video_carousel::VideoCarousel;
use adw::gtk;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use libadwaita as adw;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/GnomeTikTok/ui/master-window.ui")]
    pub struct MasterWindow {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub player_stack_switcher: TemplateChild<adw::ViewSwitcher>,
        // This view stack is for the top-level app views.
        #[template_child]
        pub master_stack: TemplateChild<adw::ViewStack>,
        // This view stack is for the video feed carousel views.
        #[template_child]
        pub player_stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub following_feed_carousel: TemplateChild<VideoCarousel>,
        #[template_child]
        pub fyp_feed_carousel: TemplateChild<VideoCarousel>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MasterWindow {
        const NAME: &'static str = "MasterWindow";
        type Type = super::MasterWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MasterWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let _obj = self.obj();
        }
    }
    impl WidgetImpl for MasterWindow {}
    impl WindowImpl for MasterWindow {}
    impl ApplicationWindowImpl for MasterWindow {}
    impl AdwApplicationWindowImpl for MasterWindow {}
}

glib::wrapper! {
    pub struct MasterWindow(ObjectSubclass<imp::MasterWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

#[gtk::template_callbacks]
impl MasterWindow {
    pub fn new<P: glib::IsA<adw::gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    #[template_callback]
    pub fn on_home_clicked(&self) {
        adw::glib::g_debug!("hey", "hey");
    }
}
