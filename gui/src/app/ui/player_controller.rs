use super::super::App;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;

pub fn new(app: &App) -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .halign(gtk::Align::Center)
        .build();

    let volume_btn = gtk::Button::builder()
        .icon_name("audio-volume-high-symbolic")
        .tooltip_text("Volume")
        .build();

    let prev_btn = gtk::Button::builder()
        .icon_name("media-skip-backward-symbolic")
        .tooltip_text("Previous")
        .build();

    let play_btn = gtk::Button::builder()
        .icon_name("media-playback-start-symbolic")
        .tooltip_text("Play/Pause")
        .build();

    let next_btn = gtk::Button::builder()
        .icon_name("media-skip-forward-symbolic")
        .tooltip_text("Next")
        .build();

    let more_btn = gtk::Button::builder()
        .icon_name("view-more-symbolic")
        .tooltip_text("More")
        .build();

    app.add_listener(clone!(@weak play_btn => move |state| {
        let icon_name = match state.playback_status {
            mpris::PlaybackStatus::Playing => "media-playback-pause-symbolic",
            _ => "media-playback-start-symbolic",
        };
        play_btn.set_icon_name(icon_name);
    }));

    container.append(&volume_btn);
    container.append(&prev_btn);
    container.append(&play_btn);
    container.append(&next_btn);
    container.append(&more_btn);

    container
}
