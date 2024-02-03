/* application.rs
 *
 * Copyright 2024 GNOME TikTok Authors
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

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use adw::gtk as gtk;
use libadwaita as adw;

use crate::globals::*;
use crate::window::MainWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GnomeTikTok {}

    #[glib::object_subclass]
    impl ObjectSubclass for GnomeTikTok {
        const NAME: &'static str = "GnomeTikTok";
        type Type = super::GnomeTikTok;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for GnomeTikTok {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for GnomeTikTok {
        fn activate(&self) {
            let application = self.obj();
            // The activate() callback also notifies us when the user tries
            // to launch a "second instance" of the application. When they try
            // to do that, we'll just present any existing window.
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = MainWindow::new(&*application);
                window.upcast()
            };

            window.set_resizable(false);
            window.set_title(Some(APP_INFO.app_title));

            if cfg!(target_arch = "aarch64") {
                // If we're targeting arm, I'm assuming we're targeting mobile.
                window.set_maximized(true);
            } else {
                assert_eq!(
                    DESKTOP_DEFAULT_DIMENSIONS.0 as f32 / DESKTOP_DEFAULT_DIMENSIONS.1 as f32,
                    DESKTOP_VIEWPORT_RATIO,
                    "The default desktop window dimensions ratio is not 18:9.",
                );
                window.set_default_size(DESKTOP_DEFAULT_DIMENSIONS.1, DESKTOP_DEFAULT_DIMENSIONS.0);

                // Silence adwaita warnings on minimum window dimensions.
                window.set_width_request(DESKTOP_DEFAULT_DIMENSIONS.1);
                window.set_height_request(DESKTOP_DEFAULT_DIMENSIONS.0);
            }
            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for GnomeTikTok {}
    impl AdwApplicationImpl for GnomeTikTok {}
}

glib::wrapper! {
    pub struct GnomeTikTok(ObjectSubclass<imp::GnomeTikTok>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl GnomeTikTok {
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
