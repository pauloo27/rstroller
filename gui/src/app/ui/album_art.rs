use crate::app::WINDOW_WIDTH;

use super::super::App;
use anyhow::Result as AnyResult;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

pub fn new(app: &App) -> gtk::Image {
    let img = gtk::Image::builder()
        .css_name("album-art")
        .valign(gtk::Align::Fill)
        .vexpand(true)
        .hexpand(true)
        .height_request(WINDOW_WIDTH)
        .build();

    let css_provider = gtk::CssProvider::new();

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    app.add_listener(clone!(@weak img => move |state| {
        img.set_tooltip_text(state.metadata.album_name());

        if let Some(art_url) = state.metadata.art_url() {
            if art_url.starts_with("file://") {
                let path = art_url.replace("file://", "");
                apply_art(img, Path::new(&path).to_path_buf(), css_provider.clone());
            } else if art_url.starts_with("http://") || art_url.starts_with("https://") {
                handle_remote_art(img, art_url.to_string(), css_provider.clone());
            } else {
                img.set_from_icon_name(None);
            }
        }
    }));

    img
}

fn apply_art(img: gtk::Image, path: PathBuf, css_provider: gtk::CssProvider) {
    img.set_from_file(Some(&path));
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let res = || -> AnyResult<DynamicImage> {
            Ok(ImageReader::open(path)?.with_guessed_format()?.decode()?)
        }();

        match res {
            Ok(img) => {
                let resized = img.resize_exact(2, 1, image::imageops::FilterType::Nearest);

                let color1 = resized.get_pixel(0, 0);
                let color2 = resized.get_pixel(1, 0);

                let color1 = format!("#{:02x}{:02x}{:02x}", color1[0], color1[1], color1[2]);
                let color2 = format!("#{:02x}{:02x}{:02x}", color2[0], color2[1], color2[2]);

                tx.send((color1, color2)).unwrap();
            }
            Err(_) => tx
                .send(("#000000".to_string(), "#000000".to_string()))
                .unwrap(),
        }
    });

    glib::spawn_future_local(async move {
        let (color1, color2) = rx.recv().expect("Failed to receive color");
        css_provider.load_from_data(
            format!("rstroller-window {{ background-image: linear-gradient(to right, {color1} 0%, {color2} 100%); }}").as_str(),
        );
    });
}

fn handle_remote_art(img: gtk::Image, art_url: String, css_provider: gtk::CssProvider) {
    let escaped_url = urlencoding::encode(&art_url);
    let path = Path::new("/tmp")
        .join("rstroller")
        .join(format!("{}", escaped_url));

    if path.exists() {
        apply_art(img, path, css_provider);
        return;
    }

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        download_file(art_url, path, tx)
            .map_err(|e| eprintln!("Failed to download album art: {}", e))
            .unwrap();
    });

    glib::spawn_future_local(async move {
        let path = rx
            .recv()
            .expect("Failed to receive download completion signal");
        apply_art(img, path, css_provider);
    });
}

fn download_file(url: String, dist_path: PathBuf, tx: mpsc::Sender<PathBuf>) -> AnyResult<()> {
    dist_path.parent().map(std::fs::create_dir_all);

    let url = reqwest::Url::parse(&url)?;
    let mut resp = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(&dist_path)?;
    std::io::copy(&mut resp, &mut file)?;

    tx.send(dist_path)
        .expect("Failed to send download completion signal");
    Ok(())
}
