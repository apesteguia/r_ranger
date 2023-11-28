pub mod app;
pub mod fs;
pub mod ui;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut app: app::App;

    if args.len() == 1 {
        app = app::App::new().unwrap();
    } else {
        app = app::App::from(&args[1]).unwrap();
    }
    app.run().unwrap();
}
