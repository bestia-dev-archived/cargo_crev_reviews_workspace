// stdio_input_password_mod.rs
// modified from https://forge.typ3.tech/charles/scanpw

//! reads password from stdin

// crossterm = "0.17"

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal,
};

/// Attempts to read a password from standard input.  
/// Backspace and ctrl+c are NOT ALLOWED for simplicity.  
/// The replacement character * is printed.  
/// The result is either a `String` or a `crossterm::ErrorKind`.  
pub fn read_passphrase_interactively() -> crossterm::Result<String> {
    // read from env var
    match std::env::var("CREV_PASSPHRASE") {
        Ok(pw) => {
            println!(r#"Reading from env CREV_PASSPHRASE."#);
            return Ok(pw);
        }
        Err(_err) => {
            println!(
                r#"The crev reviews are cryptographically signed, so you must first enter you crev passphrase to enable the signing of your crev reviews.
Passphrase does not accept backspace or ctrl+c, just characters and Enter. 
Unlocking needs 2-3 seconds after you press Enter. Holly patience..."#
            );
            print!("Enter passphrase: ");
            let mut stdout = std::io::stdout();
            std::io::Write::flush(&mut stdout)?;
            // Enter raw mode so we can control character echoing
            terminal::enable_raw_mode()?;

            // The password
            let mut pw = String::new();
            loop {
                if let Event::Key(k) = event::read()? {
                    match k {
                        // Normal character input (and SHIFT)
                        KeyEvent {
                            code: KeyCode::Char(c),
                            modifiers,
                        } => {
                            if match modifiers {
                                KeyModifiers::NONE => true,
                                KeyModifiers::SHIFT => true,
                                // Ignore other cases
                                _ => false,
                            } {
                                execute!(stdout, Print('*'))?;
                                // Add the character to the password
                                pw.push(c);
                            }
                        }
                        // Password input completed
                        KeyEvent { code: KeyCode::Enter, .. } => break,
                        // Ignore other cases
                        _ => (),
                    }
                }
            }

            // Reset the terminal back to normal
            terminal::disable_raw_mode()?;
            println!("");
            print!("Unlocking...");
            std::io::Write::flush(&mut stdout)?;

            return Ok(pw);
        }
    }
}
