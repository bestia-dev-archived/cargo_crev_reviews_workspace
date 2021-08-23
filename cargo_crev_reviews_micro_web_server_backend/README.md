# cargo_crev_reviews_micro_web_server_backend

This is a micro-web-server intended for local use with only one local browser connected to it.  
It is the backend of the application `cargo_crev_reviews`.  
The frontend is the GUI web app that runs in the browser and is connected only to this backend.  
Together the backend and the frontend form a complete application that is cross-platform.  
They share some structs for communication that are defines in a `common` project.  
The only URL the server operates is: <http://127.0.0.1:8182/cargo_crev_reviews>

## development

the micro-server will accept only [POST](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST). It must contain [json-rpc](https://www.jsonrpc.org/specification) like:

```bash
Syntax:

--> data sent to Server
<-- data sent to Client

rpc call with named parameters:

--> {
"jsonrpc": "2.0", 
"method": "subtract", 
"params": {
    "subtrahend": 23, 
    "minuend": 42}, 
    "id": 3
    }

<-- {
"jsonrpc": "2.0", 
"result": 19, 
"id": 3
}
```

Test with this in 2 separate terminals:  

```bash
~/rustprojects/cargo_crev_reviews/target/debug/cargo_crev_reviews_micro_web_server_backend
```

In the other Linux bash terminal:

```bash
curl -d '{"jsonrpc": "2.0", "method": "subtract", "params": {"subtrahend": 23, "minuend": 42}, "id": 3}' -H 'Content-Type: application/json' http://127.0.0.1:8182/cargo_crev_reviews
```
