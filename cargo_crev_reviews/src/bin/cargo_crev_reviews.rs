// cargo_crev_reviews/src/bin/main.rs

use cargo_crev_reviews::*;

fn main() -> anyhow::Result<()> {
    println!("Starting the backend CLI for cargo_crev_reviews");

    unlock_crev_id_interactively()?;

    let host = "127.0.0.1";
    let port = "8182";
    start_web_server(host, port);

    Ok(())
}
