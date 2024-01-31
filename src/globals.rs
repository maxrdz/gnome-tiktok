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

use adw::gtk::License;
use libadwaita as adw;

pub static DESKTOP_VIEWPORT_RATIO: f32 = 18.0 / 9.0;
pub static DESKTOP_DEFAULT_DIMENSIONS: (i32, i32) = (720, 360);

pub struct AboutInformation {
    pub app_name: &'static str,
    pub app_version: &'static str,
    pub app_id: &'static str,
    pub authors: &'static [&'static str],
    pub artists: Option<&'static [&'static str]>,
    pub documenters: Option<&'static [&'static str]>,
    pub copyright: &'static str,
    pub license: &'static str,
    pub license_type: License,
    pub comments: &'static str,
}

pub static APP_INFO: AboutInformation = AboutInformation {
    app_name: "TuxTok",
    app_version: "0.1.0",
    app_id: "com.maxrdz.TuxTok",
    authors: &[
        "Max Rodriguez <me@maxrdz.com>",
        "vkill <vkill.net@gmail.com>",
    ],
    artists: None,
    documenters: None,
    copyright: "© 2024 TuxTok Authors",
    license: "GNU General Public License v3.0",
    license_type: License::Gpl30,
    comments: "A Linux mobile friendly app for using the \
    Tiktok social media platform, built with libadwaita \
    and GTK. Special thanks to vkill for developing the \
    underlying TikTok API Rust library.",
};
