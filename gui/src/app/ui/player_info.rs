use super::super::App;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;

pub fn new(app: &App) -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(5)
        .css_name("player-info-container")
        .halign(gtk::Align::Center)
        .build();

    let player_name_lbl = gtk::Label::builder().css_name("player-name-label").build();
    let player_icon_img = gtk::Image::builder().css_name("player-icon-img").build();

    container.append(&player_name_lbl);
    container.append(&player_icon_img);

    app.add_listener(
        clone!(@weak player_icon_img, @weak player_name_lbl => move |p| {
            player_name_lbl.set_text(format!("Playing in {}", p.identity).as_str());
            player_name_lbl.set_tooltip_text(Some(p.identity.as_str()));

            let icon_name = p.name.to_lowercase();
            let icon_name = icon_name.split('.').nth(3).unwrap_or("");

            if gtk::IconTheme::default().has_icon(&icon_name) {
                player_icon_img.set_from_icon_name(Some(icon_name));
            } else {
                println!("No icon found for {}", icon_name);
                player_icon_img.set_from_icon_name(Some("media-optical"));
            }
        }),
    );

    container
}
