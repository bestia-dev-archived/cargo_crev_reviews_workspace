// cargo_crev_reviews/src/bin/main.rs

use std::env;

use cargo_crev_reviews::*;

/// check if cargo_crev is installed,
/// if the program is run beside Cargo.toml,
/// if the port is TcpListener is free to bind. That means that this is the only one instance of the program running.
/// User input the passphrase for crev signing.
fn main() -> anyhow::Result<()> {
    // I don't need to check for `cargo` or Rust, because cargo_crev_reviews is installed with `cargo install`.
    // It means that cargo and Rust are already installed.

    let host_port_already_busy = host_port_is_busy();
    // check if cargo-crev is installed
    let path = home::cargo_home()?.join("bin").join("cargo-crev");
    if !path.exists() {
        cargo_crev_not_installed();
    } else if !env::current_dir()?.join("Cargo.toml").exists() {
        not_started_inside_rust_project()?;
    } else if host_port_already_busy.is_some() {
        one_instance_of_the_program_already_running(&host_port_already_busy.unwrap());
    } else {
        println!(
            r#"
{yel}WELCOME to cargo_crev_reviews from Bestia.dev!{res}

With this app you can list, edit and create your `crev` reviews inside the browser.
Crev is a "Code REView and recommendation system` authored by `dpc` and published on `https://github.com/crev-dev/cargo-crev`. 
Cargo-crev is the part of crev, that is specialized for the Rust language.
First check the reviews from other developers on https://web.crev.dev/rust-reviews/crates/`.

The crev reviews are cryptographically signed, so you must first enter you crev passphrase to enable the signing of your crev reviews.
Then this CLI opens the default browser. This is the frontend graphical (GUI) part of the app.
If the default browser does not open from WSL2, you can see my project `https://github.com/LucianoBestia/wsl_open_browser`.
"#,
            yel = *YELLOW,
            res = *RESET
        );
        unlock_crev_id_interactively()?;

        start_web_server();
    }
    Ok(())
}

fn cargo_crev_not_installed() {
    println!(
        r#"
{yel}WELCOME to cargo_crev_reviews from Bestia.dev!{res}

{red}Error: cargo-crev is not installed!{res}

Cargo_crev_reviews is a GUI wrapper around cargo-crev.
Install and configure cargo-crev in 5 easy steps.
1. Install cargo-crev:
  {green}$ cargo install cargo-crev{res}
2. Fork the crev-proof repo to your Github/Gitlab
  {green}https://github.com/crev-dev/crev-proofs/fork{res}
3. Create your CrevId:
  {green}$ cargo crev id new --url https://github.com/YOUR-USERNAME/crev-proofs{res}
Choose a passphrase. Warning: There's no way to recover your CrevID if you forget your passphrase.
4. Trust the reviews of `dpc`, the author of cargo-crev:
  {green}$ cargo crev trust --level high https://github.com/dpc/crev-proofs{res}
5. Fetch existing reviews:
  {green}$ cargo crev repo fetch trusted{res}
Done! Easy!
Read more here: https://github.com/crev-dev/cargo-crev/blob/master/cargo-crev/src/doc/getting_started.md
"#,
        yel = *YELLOW,
        red = *RED,
        res = *RESET,
        green = *GREEN
    );
}

fn not_started_inside_rust_project() -> anyhow::Result<()> {
    println!(
        r#"
{yel}WELCOME to cargo_crev_reviews from Bestia.dev!{res}

{red}Error: this program was not started inside a rust project!
There is no Cargo.toml in the current directory: {dir}{res}

Cargo_crev_reviews works best when started inside a rust project 
in the directory where the Cargo.toml file is.
"#,
        dir = env::current_dir()?.to_string_lossy(),
        yel = *YELLOW,
        red = *RED,
        res = *RESET,
    );
    Ok(())
}

fn one_instance_of_the_program_already_running(url_not_free: &str) {
    println!(
        r#"
{yel}WELCOME to cargo_crev_reviews from Bestia.dev!{res}

{red}Error: one instance of the program is already running!{res}
The listener to {url} returned an error.

"#,
        yel = *YELLOW,
        red = *RED,
        res = *RESET,
        url = url_not_free,
    );
}
