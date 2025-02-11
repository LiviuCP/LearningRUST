use std::io;

// used mainly for implementing prompt for save
pub fn execute_yes_no_cancel_cta(message: &str) -> Result<Option<bool>, ()> {
    let mut result = Ok(None);

    println!("{}", message);
    println!("(y - yes, n - no, c - cancel)?\n");

    loop {
        let mut user_input = String::new();

        if let Err(_) = io::stdin().read_line(&mut user_input) {
            result = Err(());
            break;
        }

        match user_input.trim() {
            "y" | "Y" => {
                result = Ok(Some(true));
            }
            "n" | "N" => {
                result = Ok(Some(false));
            }
            "c" | "C" => {}
            _ => {
                println!("Invalid choice! Please try again");
                continue;
            }
        }

        break;
    }

    result
}
