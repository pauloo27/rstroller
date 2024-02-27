mod ui;

use std::sync::mpsc;
use std::{cell::RefCell, rc::Rc};

use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk4 as gtk;

use common::player::{PlayerAction, PlayerState};
use std::process;
use tokio::sync::mpsc as tokio_mpsc;

const APP_ID: &str = "cafe.ndo.Rstroller";
pub const WINDOW_WIDTH: i32 = 250;

type MprisListener = dyn Fn(&PlayerState);

pub struct App {
    gtk_app: gtk::Application,
    listeners: RefCell<Vec<Box<MprisListener /*---[*/>>>,
    action_sender: RefCell<Option<mpsc::Sender<PlayerAction>>>,
}

// public interface
impl App {
    pub fn new() -> Self {
        let gtk_app = gtk::Application::builder().application_id(APP_ID).build();

        App {
            gtk_app,
            listeners: RefCell::new(Vec::new()),
            action_sender: RefCell::new(None),
        }
    }

    pub fn run(self: Rc<Self>) -> i32 {
        self.gtk_app.connect_startup(|_| Self::load_global_css());
        self.gtk_app
            .connect_activate(clone!(@weak self as app => move |_| {
                app.clone().setup_ui();
                app.listen_to_mpris();
            }));

        self.gtk_app.run().value()
    }

    pub fn add_listener<F>(&self, listener: F)
    where
        F: Fn(&PlayerState) + 'static,
    {
        self.listeners.borrow_mut().push(Box::new(listener));
    }

    fn emit_player_state(&self, state: PlayerState) {
        for listener in self.listeners.borrow().iter() {
            listener(&state);
        }
    }

    pub fn send_action(&self, action: PlayerAction) {
        let sender = self.action_sender.borrow();
        if let Some(sender) = sender.as_ref() {
            sender.send(action).unwrap();
        }
    }
}

// internal implementation
impl App {
    fn load_global_css() {
        let provider = gtk::CssProvider::new();
        provider.load_from_data(include_str!("ui/style.css"));

        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn setup_ui(self: Rc<Self>) {
        let window = gtk::ApplicationWindow::builder()
            .application(&self.gtk_app)
            .title("Rstroller")
            .default_width(WINDOW_WIDTH)
            .default_height(200)
            .build();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(20)
            .build();

        container.append(&ui::album_art::new(&self));
        container.append(&ui::track_info::new(&self));
        container.append(&ui::player_controller::new(self.clone()));

        window.set_child(Some(&container));

        window.present();
    }

    fn listen_to_mpris(self: Rc<Self>) {
        let (player_tx, mut player_rx) = tokio_mpsc::channel(1);
        let (action_tx, action_rx) = mpsc::channel();

        let player = common::player::get_preferred_player_or_first();

        match player {
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
            Ok(None) => {
                eprintln!("No player found");
                process::exit(1);
            }
            Ok(Some(player)) => {
                let player_name = player.bus_name();
                println!("Player found: {}", player_name);
                let wrapper = common::player::MprisWrapper::new(player_name.to_string());
                wrapper
                    .start_listener(player_tx)
                    .expect("Could not start listener");
                wrapper
                    .start_controller(action_rx)
                    .expect("Could not start controller");
                self.action_sender.replace(Some(action_tx));
            }
        }

        glib::spawn_future_local(clone!(@weak self as app => async move {
            while let Some(state) = player_rx.recv().await {
                    app.emit_player_state(state);
            }
            eprintln!("Player shut down");
            process::exit(0);
        }));
    }
}
