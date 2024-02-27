use std::rc::Rc;

use super::super::App;
use common::player::PlayerAction;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;

pub fn new(app: Rc<App>) -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .margin_bottom(10)
        .halign(gtk::Align::Center)
        .build();

    let volume_btn = gtk::MenuButton::builder()
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

    let more_btn = gtk::MenuButton::builder()
        .icon_name("view-more-symbolic")
        .tooltip_text("More")
        .build();

    play_btn.connect_clicked(clone!(@weak app => move |_| {
        app.send_action(PlayerAction::PlayPause);
    }));

    prev_btn.connect_clicked(clone!(@weak app => move |_| {
        app.send_action(PlayerAction::Previous);
    }));

    next_btn.connect_clicked(clone!(@weak app => move |_| {
        app.send_action(PlayerAction::Next);
    }));

    create_volume_popover(app.clone(), &volume_btn);
    create_more_popover(app.clone(), &more_btn);

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

fn create_volume_popover(app: Rc<App>, btn: &gtk::MenuButton) -> gtk::Popover {
    let scale = gtk::Scale::builder()
        .orientation(gtk::Orientation::Vertical)
        .adjustment(&gtk::Adjustment::new(0.0, 0.0, 1.0, 0.01, 0.0, 0.0))
        .inverted(true)
        .build();
    let popover = gtk::Popover::builder().child(&scale).build();
    btn.set_popover(Some(&popover));

    app.add_listener(clone!(@weak scale => move |state| {
        scale.set_value(state.volume);
    }));

    scale.connect_value_changed(clone!(@weak app => move |scale| {
        app.send_action(PlayerAction::Volume(scale.value()));
    }));

    let (w, h) = scale.size_request();
    scale.set_size_request(w, h + 100);

    popover
}

fn create_more_popover(app: Rc<App>, btn: &gtk::MenuButton) -> gtk::Popover {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .build();

    let popover = gtk::Popover::builder().child(&container).build();
    btn.set_popover(Some(&popover));

    let shuffle_btn = gtk::ToggleButton::builder()
        .icon_name("media-playlist-shuffle-symbolic")
        .tooltip_text("Toggle Shuffle")
        .build();

    let raise_btn = gtk::Button::builder()
        .icon_name("go-up")
        .tooltip_text("Raise Player")
        .build();

    raise_btn.connect_clicked(clone!(@weak app => move |_| {
        app.send_action(PlayerAction::Raise);
    }));

    shuffle_btn.connect_toggled(clone!(@weak app => move |btn| {
        app.send_action(PlayerAction::Shuffle(btn.is_active()));
    }));

    app.add_listener(clone!(@weak shuffle_btn => move |state| {
        shuffle_btn.set_active(state.shuffle);
    }));

    container.append(&shuffle_btn);
    container.append(&raise_btn);

    popover
}
