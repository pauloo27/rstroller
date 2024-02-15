use super::App;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;

pub fn new(app: &App) -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let title_lbl = gtk::Label::builder().css_name("track-title-label").build();
    let artist_lbl = gtk::Label::builder().css_name("track-artist-label").build();

    container.append(&title_lbl);
    container.append(&artist_lbl);

    app.add_listener(clone!(@weak title_lbl, @weak artist_lbl => move |p| {
        let title_txt = p.metadata.title().unwrap_or("Unknown");
        title_lbl.set_tooltip_text(Some(title_txt));
        title_lbl.set_text(common::utils::truncate_string(&title_txt, 30));

        let artist_txt = match p.metadata.artists() {
            Some(artists) => artists.join(", "),
            None => "Unknown".to_string(),
        };

        artist_lbl.set_tooltip_text(Some(&artist_txt));
        artist_lbl.set_text(common::utils::truncate_string(&artist_txt, 30));
    }));

    container
}
