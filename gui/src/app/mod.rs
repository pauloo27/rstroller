mod mpris_listener;
mod player_state;
mod track_info;

use std::{cell::RefCell, rc::Rc};

use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk4 as gtk;

pub use player_state::PlayerState;

const APP_ID: &str = "cafe.ndo.Rstroller";

type MprisListener = dyn Fn(Rc<PlayerState>);

pub struct App {
    gtk_app: gtk::Application,
    listeners: Rc<RefCell<Vec<Box<MprisListener /*---[*/>>>>,
}

// public interface
impl App {
    pub fn new() -> Self {
        let gtk_app = gtk::Application::builder().application_id(APP_ID).build();

        App {
            gtk_app,
            listeners: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn run(self: Rc<Self>) -> i32 {
        self.gtk_app
            .connect_activate(clone!(@weak self as app => move |_| {
                app.setup_ui();
                app.listen_to_mpris();
            }));

        self.gtk_app.run().value()
    }

    pub fn add_listener<F>(&self, listener: F)
    where
        F: Fn(Rc<PlayerState>) + 'static,
    {
        self.listeners.borrow_mut().push(Box::new(listener));
    }

    fn emit_player_state(&self, metadata: Rc<PlayerState>) {
        for listener in self.listeners.borrow().iter() {
            listener(metadata.clone());
        }
    }
}

impl App {
    fn setup_ui(&self) {
        let window = gtk::ApplicationWindow::builder()
            .application(&self.gtk_app)
            .title("Rstroller")
            .default_width(300)
            .default_height(150)
            .build();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(20)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        container.append(&track_info::new(&self));

        window.set_child(Some(&container));

        window.present();
    }

    fn listen_to_mpris(self: Rc<Self>) {
        let (sender, receiver) = async_channel::bounded(1);

        mpris_listener::spawn_mpris_listener(sender);

        glib::spawn_future_local(clone!(@weak self as app => async move {
            while let Ok(metadata) = receiver.recv().await {
                app.emit_player_state(Rc::new(metadata));
            }
        }));
    }
}
