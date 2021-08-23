// cargo_crev_reviews_micro_web_server_backend/src/lib.rs

use unwrap::unwrap;
use simple_server::{Method, Server, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{Result};

#[derive(Serialize, Deserialize, Debug)]
struct RpcMethod{
    jsonrpc:String,
    method:String,
    params:serde_json::Value
}

pub fn start_web_server(host: &str, port: &str) {
    println!("cargo_crev_reviews_micro_web_server_backend started");
    let server = Server::new(|request, response| {
        //println!("Request received. {} {}", request.method(), request.uri());

        match request.method() {
            &Method::GET => {
                // I will use only POST, never GET
                // let body = format!("The path you requested was '{}'", request.uri().path());                 
                // Ok(response.body(body.into_bytes())?)
                Ok(response.body(b"<h1>404</h1><p>Not found!<p>".to_vec())?)
            }
            &Method::POST => {
                let data = String::from_utf8_lossy(request.body()).into_owned();
                let body = parse_post_data_and_match_method(&data);
                Ok(response.body(body.into_bytes())?)
            }
            _ => {
                let response = response.status(StatusCode::NOT_FOUND);
                Ok(response.body(b"<h1>404</h1><p>Not found!<p>".to_vec())?)
            }
        }
    });
    server.listen(host, port);
}

fn parse_post_data_and_match_method(data:&str)->String{
    //println!("data: {}", data);
    let p: RpcMethod = unwrap!(serde_json::from_str(data));
    //println!("deserialized = {:?}", &p);
    let mut body="unknown data params".to_string();
    if p.jsonrpc != "2.0"{
        body = format!("error: jsonrpc != 2.0");    
    } else {
        body = match p.method.as_str(){
            "subtract" => subtract(p.params),
            _ => format!("unknown method = {}", &p.method)

        };        
    }
    // return
    body
}

#[derive(Serialize, Deserialize, Debug)]
struct SubtractParams{
    subtrahend:f64, 
    minuend:f64
}

fn subtract(params:serde_json::Value) -> String {
    //println!("SubtractParams: {}", &params);
    let p: SubtractParams = unwrap!(serde_json::from_value(params));
    println!("SubtractParams = {:?}", &p);
    let subtraction = p.subtrahend - p.minuend;
    let body=format!("subtraction = {}", &subtraction);
    // return
    body
}