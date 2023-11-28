pub mod app;
pub mod fs;
pub mod ui;

fn main() {
    let mut app = app::App::new().unwrap();
    app.run().unwrap();
}
