

use env_logger;
use cargo_crev_reviews_micro_web_server_backend::start_web_server;

fn main() {
    env_logger::init();

    let host = "127.0.0.1";
    let port = "8182";

    start_web_server(host, port);
}
