/* application.rs
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
use cfg_if::cfg_if;
use gtk::prelude::*;
use gtk::{gio, glib};
use libadwaita as adw;

use crate::globals::*;
use crate::master_window::MasterWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GnomeTuxTok {}

    #[glib::object_subclass]
    impl ObjectSubclass for GnomeTuxTok {
        const NAME: &'static str = "GnomeTuxTok";
        type Type = super::GnomeTuxTok;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for GnomeTuxTok {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for GnomeTuxTok {
        fn activate(&self) {
            let application = self.obj();
            // The activate() callback also notifies us when the user tries
            // to launch a "second instance" of the application. When they try
            // to do that, we'll just present any existing window.
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = MasterWindow::new(&*application);
                window.upcast()
            };

            window.set_title(Some(APP_INFO.app_title));

            cfg_if!(
                // If we're targeting arm, I'm assuming we're targeting mobile.
                if #[cfg(target_arch = "aarch64")] {
                    window.set_fullscreened(true);
                    // if we go out of focus, once we come back, restore fullscreen again.
                    window.connect_focus_visible_notify(move |win| win.set_fullscreened(true));
                } else {
                    window.set_maximized(true);

                    window.connect_maximized_notify(move |win| {
                        //let screen_width: i32 = win.width();
                        let screen_width: i32 = 1080; // issue with .width() ^^
                        win.set_maximized(false);

                        let window_width: i32 = screen_width / DESKTOP_WIDTH_DIVISOR;
                        let height_float: f32 = window_width as f32 * VIEWPORT_ASPECT_RATIO;
                        let window_height: i32 = height_float.round() as i32;

                        win.set_default_size(window_width, window_height);

                        // Silence adwaita warnings on minimum window dimensions.
                        win.set_width_request(window_width);
                        win.set_height_request(window_height);
                        win.set_resizable(false);
                    })
                }
            );
            window.present();
        }
    }

    impl GtkApplicationImpl for GnomeTuxTok {}
    impl AdwApplicationImpl for GnomeTuxTok {}
}

glib::wrapper! {
    pub struct GnomeTuxTok(ObjectSubclass<imp::GnomeTuxTok>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl GnomeTuxTok {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        // No exit/back button on this window, so mobile users
        // (at least on Phosh) need to swipe out of the program
        // and close this new tab to return back to the main window.
        let about = adw::AboutWindow::builder()
            .transient_for(&window)
            .modal(true)
            .application_icon(APP_INFO.app_id)
            .application_name(APP_INFO.app_title)
            .developer_name(*APP_INFO.authors.first().unwrap())
            .version(APP_INFO.app_version)
            .issue_url(format!("{}/issues", APP_INFO.app_repo).as_str())
            .developers(APP_INFO.authors)
            .copyright(APP_INFO.copyright)
            .license(APP_INFO.license)
            .license_type(APP_INFO.license_type)
            .comments(APP_INFO.comments)
            .build();
        about.present();
    }
}
