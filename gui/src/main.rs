mod app;

use app::App;
use std::{process, rc::Rc};

fn main() {
    let app = Rc::new(App::new());
    let code = app.run();
    process::exit(code);
}
