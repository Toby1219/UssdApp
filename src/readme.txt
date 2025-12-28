USSD CLI application library (core logic)

Overview
This file implements the core logic for a simple USSD-style CLI banking app. It provides:
- A small state machine for menu navigation (State)
- A Menus enum to map numeric menu options to labels
- An App struct that holds application state (user name, age, balance, interest, transaction history)
- Interactive I/O helpers and operations for debit/transactions
- Unit tests that validate menu mapping, age verification, debit logic and state transitions

Public API
- pub struct App
  - Construct with: App::new(name: &str, age: i32) -> Option<App>
    - Returns None when the age verification fails (age < 18).
  - Methods:
    - pub fn new(name: &str, age: i32) -> Option<App>
    - pub fn input(msg: &str) -> String
      - Simple stdin prompt helper (prints msg and returns trimmed input).
    - pub fn run(&mut self)
      - Starts the interactive loop. Presents the menu, reads input and transitions states until Exit.

Behavior details and defaults
- Starting balance: 1000.0
- Starting interest: 0.0
- History: Vec<String>, updated by update_history on transactions
- Menu options are 0..=5:
  - 0: Exit
  - 1: Account details
  - 2: Balance
  - 3: Transfer
  - 4: Buy Airtime
  - 5: Buy Data
- Age verification:
  - App::new returns None for ages under 18 and prints "Too Young to open account !!!"
- Debit logic:
  - Rejects non-positive amounts (prints "Invalid ammount").
  - Fails when amount >= balance (prints "Insuffcient Funds ..." and records a failed entry).
  - On success, deducts amount, records success and prints confirmation.
- Note: Many user-facing strings and identifiers contain typographical errors (e.g., "AcoountDetails", "Balnce", "intrest", "ammount", "sucessfull"). These affect test expectations and history messages.

Examples

1) Interactive usage (binary that uses this library)
- Typical main.rs:
  fn main() {
      let mut app = match ussd_app::App::new("Alice", 25) {
          Some(a) => a,
          None => return,
      };
      app.run();
  }

2) Programmatic usage (non-interactive)
- Directly call methods (useful for tests or embedding in another service):
  let mut app = ussd_app::App::new("Alice", 25).unwrap();
  app.debit("Transfer", 200.0); // reduces balance by 200 or records failure

Running tests
- From the repository root:
  cargo test --lib
- Tests in this file validate:
  - Menus::from_number mapping
  - App::verify_age behavior
  - App::new initialization
  - debit() success/failure cases
  - state-return-to-menu behavior for account/balance states


Notes about contributors
- The unit tests in this file already provide a good starting point for verifying refactors. If you rename identifiers or change behavior, update tests accordingly.


