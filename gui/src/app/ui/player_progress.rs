use super::super::App;
use common::player::PlayerAction;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use std::rc::Rc;
use std::time::Duration;

pub fn new(app: Rc<App>) -> gtk::Scale {
    let scale = gtk::Scale::builder()
        .orientation(gtk::Orientation::Horizontal)
        .adjustment(&gtk::Adjustment::new(0.0, 0.0, 1.0, 0.01, 0.0, 0.0))
        .css_classes(vec!["player-progress-scale"])
        .build();

    scale.connect_value_changed(clone!(
        #[weak]
        app,
        move |scale| {
            let state = app.most_recent_state.borrow();
            if let Some(state) = state.as_ref() {
                let new_duration =
                    state.metadata.length_in_microseconds().unwrap_or(0) as f64 * scale.value();
                let new_duration = Duration::from_micros(new_duration as u64);
                app.send_action(PlayerAction::Seek(new_duration));
            }
        }
    ));

    scale
}
