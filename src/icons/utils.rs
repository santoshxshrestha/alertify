use super::action::STD_ACTION_ICONS;
use super::animation::STD_ANIMATION_ICONS;
use super::application::STD_APPLICATION_ICONS;
use super::category::STD_CATEGORY_ICONS;
use super::device::STD_DEVICE_ICONS;
use super::emblem::STD_EMBLEM_ICONS;
use super::emotion::STD_EMOTION_ICONS;
use super::international::STD_INTERNATIONAL_ICONS;
use super::mime::STD_MIME_TYPE_ICONS;
use super::place::STD_PLACE_ICONS;
use super::status::STD_STATUS_ICONS;

use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum IconSet {
    All,
    Actions,
    Animations,
    Applications,
    Categories,
    Devices,
    Emblems,
    Emotes,
    International,
    MimeTypes,
    Places,
    Status,
}

pub fn handle_icon_listing(set: IconSet) {
    let mut lists = Vec::new();
    match set {
        IconSet::All => {
            lists.push(("actions", STD_ACTION_ICONS));
            lists.push(("animations", STD_ANIMATION_ICONS));
            lists.push(("applications", STD_APPLICATION_ICONS));
            lists.push(("categories", STD_CATEGORY_ICONS));
            lists.push(("devices", STD_DEVICE_ICONS));
            lists.push(("emblems", STD_EMBLEM_ICONS));
            lists.push(("emotes", STD_EMOTION_ICONS));
            lists.push(("international", STD_INTERNATIONAL_ICONS));
            lists.push(("mimetypes", STD_MIME_TYPE_ICONS));
            lists.push(("places", STD_PLACE_ICONS));
            lists.push(("status", STD_STATUS_ICONS));
        }
        IconSet::Actions => lists.push(("actions", STD_ACTION_ICONS)),
        IconSet::Animations => lists.push(("animations", STD_ANIMATION_ICONS)),
        IconSet::Applications => lists.push(("applications", STD_APPLICATION_ICONS)),
        IconSet::Categories => lists.push(("categories", STD_CATEGORY_ICONS)),
        IconSet::Devices => lists.push(("devices", STD_DEVICE_ICONS)),
        IconSet::Emblems => lists.push(("emblems", STD_EMBLEM_ICONS)),
        IconSet::Emotes => lists.push(("emotes", STD_EMOTION_ICONS)),
        IconSet::International => lists.push(("international", STD_INTERNATIONAL_ICONS)),
        IconSet::MimeTypes => lists.push(("mimetypes", STD_MIME_TYPE_ICONS)),
        IconSet::Places => lists.push(("places", STD_PLACE_ICONS)),
        IconSet::Status => lists.push(("status", STD_STATUS_ICONS)),
    }

    for (label, icons) in lists {
        println!("[{label}]");
        for (icon_name, description) in icons {
            println!("{}: {}", icon_name, description);
        }
        println!();
    }
}
