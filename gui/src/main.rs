mod app;

use app::App;
use std::{process, rc::Rc};

fn main() {
    let app = Rc::new(App::new());
    app.clone().listen_to_mpris();
    let code = app.run();
    process::exit(code);
}
