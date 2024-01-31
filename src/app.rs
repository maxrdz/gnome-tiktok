// Copyright (c) 2024, Gtktok Authors.

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 3.
// You should have received a copy of this license along
// with this source code in a file named "LICENSE."
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software Foundation,
// Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use crate::globals::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

pub struct GtktokApp {
    gtk_window: ApplicationWindow,
}

impl GtktokApp {
    pub fn new(gtk_app: &Application) -> Self {
        Self {
            gtk_window: ApplicationWindow::new(gtk_app),
        }
    }

    pub fn gtk_win(&self) -> &ApplicationWindow {
        &self.gtk_window
    }

    pub fn initialize(&self) {
        assert_eq!(
            DESKTOP_DEFAULT_DIMENSIONS.0 as f32 / DESKTOP_DEFAULT_DIMENSIONS.1 as f32,
            DESKTOP_VIEWPORT_RATIO,
            "The default desktop window dimensions ratio is not 20:9.",
        );

        self.gtk_win()
            .set_default_size(DESKTOP_DEFAULT_DIMENSIONS.1, DESKTOP_DEFAULT_DIMENSIONS.0);

        self.on_about();
        let button = gtk::Button::with_label("Hello World!");
        self.gtk_win().set_child(Some(&button));
        self.gtk_win().present();
    }

    pub fn on_about(&self) {
        // NOTE: not using the builder pattern due to different argument types.
        let about_dialog: gtk::AboutDialog = gtk::AboutDialog::new();

        about_dialog.set_program_name(APP_INFO.app_name);
        about_dialog.set_version(APP_INFO.app_version);
        about_dialog.set_authors(APP_INFO.authors);
        about_dialog.set_artists(APP_INFO.artists);
        about_dialog.set_documenters(APP_INFO.documenters);
        about_dialog.set_copyright(APP_INFO.copyright);
        about_dialog.set_license(APP_INFO.license);
        about_dialog.set_license_type(APP_INFO.license_type);
        about_dialog.set_comments(APP_INFO.comments);
        about_dialog.show();
    }
}
