use crate::app::WINDOW_WIDTH;

use super::super::App;
use anyhow::Result as AnyResult;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;

pub fn new(app: &App) -> gtk::Image {
    let img = gtk::Image::builder()
        .css_name("album-art")
        .valign(gtk::Align::Start)
        .height_request(WINDOW_WIDTH)
        .build();

    app.add_listener(clone!(@weak img => move |state| {
        img.set_tooltip_text(state.metadata.album_name());

        if let Some(art_url) = state.metadata.art_url() {
            if art_url.starts_with("file://") {
                let path = art_url.replace("file://", "");
                img.set_from_file(Some(&path));
            } else if art_url.starts_with("http://") || art_url.starts_with("https://") {
                handle_remote_art(img, art_url.to_string());
            } else {
                img.set_from_icon_name(None);
            }
        }
    }));

    img
}

fn handle_remote_art(img: gtk::Image, art_url: String) {
    let escaped_url = urlencoding::encode(&art_url);
    let path = Path::new("/tmp")
        .join("rstroller")
        .join(format!("{}", escaped_url));

    if path.exists() {
        img.set_from_file(Some(&path));
        return;
    }

    let (tx, rx) = mpsc::channel();

    download_file(art_url, path, tx)
        .map_err(|e| eprintln!("Failed to download album art: {}", e))
        .unwrap();

    glib::spawn_future_local(async move {
        let path = rx
            .recv()
            .expect("Failed to receive download completion signal");
        img.set_from_file(Some(path));
    });
}

fn download_file(url: String, dist_path: PathBuf, tx: mpsc::Sender<PathBuf>) -> AnyResult<()> {
    dist_path.parent().map(|p| std::fs::create_dir_all(p));

    let url = reqwest::Url::parse(&url)?;
    let mut resp = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(&dist_path)?;
    std::io::copy(&mut resp, &mut file)?;

    tx.send(dist_path)
        .expect("Failed to send download completion signal");
    Ok(())
}
