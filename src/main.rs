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

mod app;
mod globals;

use adw::glib::ExitCode;
use adw::prelude::*;
use adw::Application;
use globals::APP_INFO;
use libadwaita as adw;

fn on_activate(adw_application: &Application) {
    let app: app::GtktokApp = app::GtktokApp::new(adw_application);
    app.initialize();
}

fn main() -> Result<(), ()> {
    let adw_app = Application::builder()
        .application_id(APP_INFO.app_id)
        .build();
    adw_app.connect_activate(on_activate);

    match adw_app.run() {
        ExitCode::SUCCESS => Ok(()),
        ExitCode::FAILURE => Err(()),
        _ => Err(()),
    }
}
