// cargo_crev_reviews/src/bin/main.rs

use cargo_crev_reviews::start_web_server;

fn main() {
    let host = "127.0.0.1";
    let port = "8182";

    start_web_server(host, port);
}
