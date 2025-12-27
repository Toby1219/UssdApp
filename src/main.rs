use ussdapp::App;

fn main() {
    let mut app = App::new("Tobi", 28).expect("Failed to ceate account");
    app.run();
}
