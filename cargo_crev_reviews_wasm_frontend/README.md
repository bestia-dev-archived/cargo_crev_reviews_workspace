# cargo_crev_reviews_wasm_frontend

This simple web app is the GUI frontend of the application `cargo_crev_reviews`.  
It is strictly designed for use on desktops as it is a tool for programers. No need to do a mobile version.  
Together the backend and the frontend form a complete application that is cross-platform.  
They share some structs for communication that are defines in a `common` project.  
The only URL to talk to the backend is: <http://127.0.0.1:8182/cargo_crev_reviews>
The micro-server accepts only [POST](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST).  
It must contain a json.  
I think it is called [json-rpc](https://www.jsonrpc.org/specification) like:

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
