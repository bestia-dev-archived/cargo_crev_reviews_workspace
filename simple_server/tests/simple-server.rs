
use simple_server::Server;

#[test]
fn test_server_new() {
    Server::new(|_request, response| Ok(response.body("Hello Rust!(test_server_new)".as_bytes().to_vec())?));
}

#[test]
fn test_error_fallback() {
    Server::new(|_request, response| {
        // set an invalid header
        let response = response.header("Foo", "Bar\r\n");

        // this will then fail
        Ok(response.body("".as_bytes().to_vec())?)
    });
}
