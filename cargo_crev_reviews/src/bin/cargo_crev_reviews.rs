// cargo_crev_reviews/src/bin/main.rs

use cargo_crev_reviews::*;

fn main() -> anyhow::Result<()> {
    println!(
        r#"
{yel}WELCOME to cargo_crev_reviews from Bestia.dev!{res}
With this app you can list, edit and create your `crev` reviews inside the browser.
Crev is a "Code REView and recommendation system` authored by `dpc` and published on `https://github.com/crev-dev/cargo-crev`. 
Cargo-crev is the part of crev, that is specialized for the Rust language.
First check the reviews from other developers on https://web.crev.dev/rust-reviews/crates/`.
Cargo-crev is based on a CLI application. Follow the `Getting started guide` https://github.com/crev-dev/cargo-crev/blob/master/cargo-crev/src/doc/getting_started.md.
Once you have installed the CLI, created your crev-ID and forked the `crev-proofs` repo, you are ready to write your crev reviews.
You can review only crate versions that you actually use in your projects. 
Cargo downloads the source code of this crates from crates.io into ~/.cargo/registry/src/github.com-1ecc6299db9ec823/.
You can open this local directory and examine the exact code for the crate that is compiled into your projects.
Crates.io guarantees that this code is immutable for a crate version. So you know that you are really reviewing the right code.

This app `cargo_crev_reviews` is just a wrapper around `cargo-crev` to enable work in a GUI. It is made of 2 parts. 
The backend simple web server CLI runs in Linux. On Win10 you can run it inside WSL2. It responds only for local requests on 127.0.0.1:8182.
It reads from directory `~/.cargo/registry/src/github.com-1ecc6299db9ec823/` and writes into `~/.config/crev/proofs/<repo>_<owner>_crev-proofs-<id>/<id>/reviews`.
It uses the libraries `crev-lib` and `crev-data` just like the commands from the `cargo-crev` CLI.
The crev reviews are cryptographically signed, so you must first enter you crev passphrase to enable the signing of your crev reviews.
Then this CLI opens the default browser. This is the frontend graphical (GUI) part of the app, where you list, edit and create new reviews.
If the default browser does not open from WSL2, you can see my project `https://github.com/LucianoBestia/wsl_open_browser`.
"#,
        yel = *YELLOW,
        res = *RESET
    );
    unlock_crev_id_interactively()?;

    let host = "127.0.0.1";
    let port = "8182";
    start_web_server(host, port);

    Ok(())
}
