/* master_window.rs
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

use crate::feed_carousel::FeedCarousel;
use adw::gtk;
use adw::glib::ObjectExt;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use libadwaita as adw;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/maxrdz/GnomeTuxTok/ui/master-window.ui")]
    pub struct MasterWindow {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub master_stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub player_stack_switcher: TemplateChild<adw::ViewSwitcher>,
        #[template_child]
        pub session_switcher_bar: TemplateChild<adw::ViewSwitcherBar>,
        // This view stack is for the top-level app views.
        #[template_child]
        pub session_stack: TemplateChild<adw::ViewStack>,
        // This view stack is for the video feed carousel views.
        #[template_child]
        pub player_stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub following_feed_carousel: TemplateChild<FeedCarousel>,
        #[template_child]
        pub fyp_feed_carousel: TemplateChild<FeedCarousel>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MasterWindow {
        const NAME: &'static str = "MasterWindow";
        type Type = super::MasterWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl MasterWindow {
        #[template_callback]
        fn on_session_stack_visible(&self) {
            self.player_stack_switcher.set_property("visible", "True");
            self.session_switcher_bar.set_property("reveal", "True");
        }
    }

    impl ObjectImpl for MasterWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let _obj = self.obj();

            // FIXME: This is not necessary, just for looks, but it changes the stack
            // view page icon to a filled version if it is currently displayed. When
            // I set the `icon-name` property, it sets the icon of the literal page
            // content instead. When I try to access the page content's parent, which
            // **should** be the ViewStackPage, instead I get the ViewStack itself.
            // I have no idea why I can't change the icon of the ViewStackPage.
            //self.master_stack.connect_visible_child_notify(
            //    move |view_stack: &'_ adw::ViewStack| {
            //        let vc_res: Option<gtk::Widget> = view_stack.visible_child();
            //        assert_eq!(
            //            vc_res.is_some(),
            //            true,
            //            "visible_child_notify signal received, but no visible child found."
            //        );
            //        let visible_child: gtk::Widget = vc_res.unwrap();
            //        let mut page_name: String = visible_child.property("name");
            //        page_name.push_str("-symbolic");
            //        visible_child.set_property("icon-name", page_name.as_str());
            //    },
            //);
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

impl MasterWindow {
    pub fn new<P: glib::IsA<adw::gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
