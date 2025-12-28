use ussdapp::App;

fn main() {
    let name = App::input("Enter your name: ");
    let age = App::input("Enter your age (please note you must be above 18 years): ")
        .parse::<i32>()
        .ok()
        .unwrap();

    let mut app = match App::new(&name, age) {
        Some(v) => v,
        None => {
            eprintln!("Account creation for {} faild", name);
            return;
        }
    };
    app.run();
}
