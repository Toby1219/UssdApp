// This is a known design  state  using Enum
use std::io;

#[derive(Debug, PartialEq)]
enum State {
    Menu,
    AcountDetails,
    Balance,
    Transfer,
    Airtime,
    Data,
    Exit,
}

#[derive(Debug, PartialEq)]
enum Menus {
    One,
    Two,
    Three,
    Four,
    Five,
    Zero,
}

impl Menus {
    fn label(&self) -> &'static str {
        match self {
            Menus::One => "Account details",
            Menus::Two => "Balance",
            Menus::Three => "Transfer",
            Menus::Four => "Buy Airtime",
            Menus::Five => "Buy Data",
            Menus::Zero => "Exit",
        }
    }

    fn from_number(n: u32) -> Option<Self> {
        match n {
            0 => Some(Menus::Zero),
            1 => Some(Menus::One),
            2 => Some(Menus::Two),
            3 => Some(Menus::Three),
            4 => Some(Menus::Four),
            5 => Some(Menus::Five),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct App {
    state: State,
    valid: bool,
    name: String,
    age: i32,
    bal: f64,
    intrest: f64,
    history: Vec<String>,
}

impl App {
    pub fn new(name: &str, age: i32) -> Option<App> {
        let valid_age = match App::verify_age(age) {
            Some(v) => v,
            None => return None,
        };
        Some(App {
            state: State::Menu,
            valid: valid_age,
            name: name.to_string(),
            age,
            bal: 1000.0,
            intrest: 0.0,
            history: vec![],
        })
    }

    fn verify_age(age: i32) -> Option<bool> {
        if age >= 18 {
            Some(true)
        } else {
            println!("Too Young to open account !!!",);
            return None;
        }
    }

    pub fn input(msg: &str) -> String {
        let mut buf = String::new();
        println!("{}", msg);
        io::stdin().read_line(&mut buf).expect("Invalid input");
        buf.trim().to_string()
    }

    pub fn run(&mut self) {
        loop {
            self.state = match self.state {
                State::Menu => self.menu_state(),
                State::AcountDetails => self.account_state(),
                State::Balance => self.balance_state(),
                State::Transfer => self.transfer_state(),

                State::Airtime => self.airtime_state(),
                State::Data => self.data_state(),
                State::Exit => {
                    println!("Quitting App ...");
                    break;
                }
            };
        }
    }

    fn menu_state(&self) -> State {
        println!("\n\n\t ==== USSD CLI APP ===\n");
        println!("\tWelcome {}", self.name.to_uppercase());
        println!("Please input 0 to 5");

        for i in 0..=5 {
            let menu = Menus::from_number(i).unwrap();
            println!("{}. {}", i, menu.label());
        }

        let input = App::input("\nChoose option:").parse::<u32>().ok();

        match input.and_then(Menus::from_number) {
            Some(Menus::Zero) => State::Exit,
            Some(Menus::One) => State::AcountDetails,
            Some(Menus::Two) => State::Balance,
            Some(Menus::Three) => State::Transfer,
            Some(Menus::Four) => State::Airtime,
            Some(Menus::Five) => State::Data,
            None => {
                println!("Invalid Option");
                State::Menu
            }
        }
    }

    fn account_state(&mut self) -> State {
        println!(
            "\nName: {}\nAge: {}\n\t---- Balance: ${} ----",
            self.name, self.age, self.bal
        );
        State::Menu
    }

    fn balance_state(&mut self) -> State {
        self.state = State::Balance;
        println!("\nBalance: {}", self.bal);
        println!("Interst: {}", self.intrest);
        println!("\n**---- HISTORY ----**\n");
        if self.history.is_empty() {
            println!("No Transaction history");
        } else {
            for (i, h) in self.history.iter().enumerate() {
                println!("{}. {}", i + 1, h);
            }
        }
        State::Menu
    }

    fn transfer_state(&mut self) -> State {
        let amt = App::input("Enter Ammount").parse::<f64>().unwrap_or(0.0);
        self.debit("Transfer", amt);
        State::Menu
    }

    fn airtime_state(&mut self) -> State {
        self.state = State::Airtime;
        let amt = App::input("Enter airtime amount")
            .parse::<f64>()
            .unwrap_or(0.0);
        self.debit("Airtime purchase", amt);
        State::Menu
    }

    fn data_state(&mut self) -> State {
        self.state = State::Data;
        let amt = App::input("Enter data amount")
            .parse::<f64>()
            .unwrap_or(0.0);

        self.debit("Data purchase", amt);
        State::Menu
    }

    /*-------------- LOGIC -----------------*/

    fn debit(&mut self, action: &str, amt: f64) {
        if amt <= 0.0 {
            println!("Invalid amount");
            return;
        }
        if amt >= self.bal {
            println!("Insufficient Funds ...");
            self.update_history(action, amt, false);
            return;
        }
        self.bal -= amt;
        self.update_history(action, amt, true);
        println!("{} successful!", action);
    }

    fn update_history(&mut self, action: &str, amt: f64, status: bool) {
        if status {
            self.history
                .push(format!("{} of ${} successfull", action, amt))
        } else {
            self.history.push(format!("{} of ${} failed", action, amt))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* ---------- MENUS TESTS ---------- */

    #[test]
    fn menu_from_number_valid() {
        assert_eq!(Menus::from_number(0), Some(Menus::Zero));
        assert_eq!(Menus::from_number(1), Some(Menus::One));
        assert_eq!(Menus::from_number(2), Some(Menus::Two));
        assert_eq!(Menus::from_number(3), Some(Menus::Three));
        assert_eq!(Menus::from_number(4), Some(Menus::Four));
        assert_eq!(Menus::from_number(5), Some(Menus::Five));
    }

    #[test]
    fn menu_from_number_invalid() {
        assert_eq!(Menus::from_number(6), None);
        assert_eq!(Menus::from_number(99), None);
    }

    /* ---------- AGE VERIFICATION ---------- */

    #[test]
    fn verify_age_accepts_valid_age() {
        assert_eq!(App::verify_age(18), Some(true));
        assert_eq!(App::verify_age(25), Some(true));
    }

    //#[test]
    //fn verify_age_rejects_underage() {
    //assert_eq!(App::verify_age(17), None);
    //}

    /* ---------- APP CREATION ---------- */

    #[test]
    fn app_creation_success() {
        let app = App::new("Tobi", 22);
        assert!(app.is_some());

        let app = app.unwrap();
        assert_eq!(app.state, State::Menu);
        assert_eq!(app.bal, 1000.0);
        assert_eq!(app.intrest, 0.0);
        assert!(app.history.is_empty());
    }

    //#[test]
    //fn app_creation_fails_for_underage() {
    //let app = App::new("Tobi", 16);
    //assert!(app.is_none());
    //}

    /* ---------- DEBIT LOGIC ---------- */

    #[test]
    fn debit_successfully_reduces_balance_and_updates_history() {
        let mut app = App::new("Tobi", 22).unwrap();

        app.debit("Transfer", 200.0);

        assert_eq!(app.bal, 800.0);
        assert_eq!(app.history.len(), 1);
        assert!(app.history[0].contains("Transfer"));
        assert!(app.history[0].contains("successfull"));
    }

    #[test]
    fn debit_fails_when_insufficient_funds() {
        let mut app = App::new("Tobi", 22).unwrap();

        app.debit("Transfer", 2000.0);

        assert_eq!(app.bal, 1000.0);
        assert_eq!(app.history.len(), 1);
        assert!(app.history[0].contains("failed"));
    }

    #[test]
    fn debit_rejects_zero_or_negative_amount() {
        let mut app = App::new("Tobi", 22).unwrap();

        app.debit("Transfer", 0.0);
        app.debit("Transfer", -100.0);

        assert_eq!(app.bal, 1000.0);
        assert!(app.history.is_empty());
    }

    /* ---------- STATE TRANSITIONS ---------- */

    #[test]
    fn account_state_returns_to_menu() {
        let mut app = App::new("Tobi", 22).unwrap();

        let next_state = app.account_state();

        assert_eq!(next_state, State::Menu);
    }

    #[test]
    fn balance_state_returns_to_menu() {
        let mut app = App::new("Tobi", 22).unwrap();

        let next_state = app.balance_state();

        assert_eq!(next_state, State::Menu);
    }

    #[test]
    fn history_updates_after_successful_transaction() {
        let mut app = App::new("Tobi", 22).unwrap();

        app.debit("Airtime purchase", 100.0);

        assert_eq!(app.history.len(), 1);
        assert!(app.history[0].contains("Airtime purchase"));
    }
}
