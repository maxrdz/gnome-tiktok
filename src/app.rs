// Copyright (c) 2024, TuxTok Authors.

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
use adw::prelude::*;
use adw::{Application, ApplicationWindow};
use libadwaita as adw;

pub struct GtktokApp {
    adw_window: ApplicationWindow,
}

impl GtktokApp {
    pub fn new(adw_app: &Application) -> Self {
        Self {
            adw_window: ApplicationWindow::new(adw_app),
        }
    }

    pub fn window(&self) -> &ApplicationWindow {
        &self.adw_window
    }

    pub fn initialize(&self) {
        assert_eq!(
            DESKTOP_DEFAULT_DIMENSIONS.0 as f32 / DESKTOP_DEFAULT_DIMENSIONS.1 as f32,
            DESKTOP_VIEWPORT_RATIO,
            "The default desktop window dimensions ratio is not 18:9.",
        );

        self.window()
            .set_default_size(DESKTOP_DEFAULT_DIMENSIONS.1, DESKTOP_DEFAULT_DIMENSIONS.0);

        self.on_about();
        self.window().present();
    }

    pub fn on_about(&self) {
        let about_dialog: adw::AboutWindow = adw::AboutWindow::new();

        about_dialog.set_application_name(APP_INFO.app_name);
        about_dialog.set_version(APP_INFO.app_version);
        about_dialog.set_developers(APP_INFO.authors);
        // TODO: libadwaiita doesnt accept options, but gtk does. could contribute?
        if let Some(artists) = APP_INFO.artists {
            about_dialog.set_artists(artists);
        }
        if let Some(documenters) = APP_INFO.documenters {
            about_dialog.set_documenters(documenters);
        }
        about_dialog.set_copyright(APP_INFO.copyright);
        about_dialog.set_license(APP_INFO.license);
        about_dialog.set_license_type(APP_INFO.license_type);
        about_dialog.set_comments(APP_INFO.comments);
        about_dialog.set_transient_for(Some(self.window()));
        about_dialog.present();
    }
}
