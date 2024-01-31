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

use gtk::License;

pub static APP_VERSION: &str = "0.1.0";
pub static APP_ID: &str = "com.maxrdz.Gtktok";

pub static DESKTOP_VIEWPORT_RATIO: f32 = 20.0 / 9.0;
pub static DESKTOP_DEFAULT_DIMENSIONS: (i32, i32) = (800, 360);

pub struct AboutInformation {
    pub app_name: Option<&'static str>,
    pub app_version: Option<&'static str>,
    pub authors: &'static [&'static str],
    pub artists: &'static [&'static str],
    pub documenters: &'static [&'static str],
    pub copyright: Option<&'static str>,
    pub license: Option<&'static str>,
    pub license_type: License,
    pub comments: Option<&'static str>,
}

pub static APP_INFO: AboutInformation = AboutInformation {
    app_name: Some("gtktok"),
    app_version: Some(APP_VERSION),
    authors: &["Max Rodriguez <me@maxrdz.com>"],
    artists: &[""],
    documenters: &[""],
    copyright: Some("© 2024 Gtktok Authors"),
    license: Some("GNU General Public License v3.0"),
    license_type: License::Gpl30,
    comments: None,
};
