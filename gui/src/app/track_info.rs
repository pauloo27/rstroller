use super::App;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;

pub fn new(app: &App) -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let title_lbl = gtk::Label::new(None);
    let artist_lbl = gtk::Label::new(None);

    container.append(&title_lbl);
    container.append(&artist_lbl);

    app.add_listener(
        clone!(@weak title_lbl, @weak artist_lbl => move |metadata| {
            title_lbl.set_text(metadata.title().unwrap_or("Unknown"));
            let artist = match metadata.artists() {
                Some(artists) => artists.join(", "),
                None => "Unknown".to_string(),
            };

            artist_lbl.set_text(&artist);
        }),
    );

    container
}
