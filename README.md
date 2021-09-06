[comment]: # (auto_md_to_doc_comments segment start A)

# cargo_crev_reviews

[comment]: # (auto_cargo_toml_to_md start)

Write cargo-crev reviews in a Graphical User Interface

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1336-green.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-274-blue.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-97-purple.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-96-yellow.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-140-orange.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)

[comment]: # (auto_lines_of_code end)

[comment]: # (auto_badges end)

[comment]: # (auto_badges end)

## Motivation

I think [cargo-crev](https://lib.rs/crates/cargo-crev) is a great tool to express trustworthiness in the open-source community, especially for the [rust programming language](https://www.rust-lang.org/).  I fear so much of [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack) using dependencies from [crates.io](https://crates.io/). For the smallest project you can get 100 dependencies easily. How to trust them all? To review them all manually? It is just crazy.  
But if enough people write reviews, it will be so much easier to trust the code. It is the same principle as [booking.com](https://www.booking.com/) or [air-bnb](https://www.airbnb.com/). Guests of a hotel write a review about their actual experience in the hotel. And you can read a hopefully truthful review and can understand if the hotel is good or bad. Sometimes can happen to find a fake review, but if there is enough people, most of them will be sincere.  
Sadly, writing reviews in `cargo-crev` is very hard if you are not a die-hard [VIM](https://www.vim.org/) user. I just cannot do that. I am a [VSCode](https://code.visualstudio.com/) guy. I don't intend to lear VIM. Sorry.  
This is a great project for me to play and learn with [rust](https://www.rust-lang.org/): let's make a [GUI](https://en.wikipedia.org/wiki/Graphical_user_interface) to write reviews for `cargo-crev`.  
And we will see walking the path what obstacles we must overcome.  

## Technical decisions

Rust does not have a true GUI story. It is mostly for [CLI](https://en.wikipedia.org/wiki/Command-line_interface) and libraries. Because GUI is mostly non cross-platform.  
But rust is the best language for [Wasm/Webassembly](https://webassembly.org/). So let's combine this.  
I will make a rust [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) (multi-projects):

1. CLI for a web server (micro-server)
2. Wasm for the browser (chrome and similar)

The web server CLI will access files, commands, libraries and maybe the network. This will work only in [Linux](https://en.wikipedia.org/wiki/Linux), but today Win10 has integrated Linux with [WSL2](https://en.wikipedia.org/wiki/Windows_Subsystem_for_Linux). It will work just fine on most common existing operating systems.  
Wasm in browser will just access to our micro web server. This is gonna be our GUI and because browser work in every [OS](https://en.wikipedia.org/wiki/Operating_system), we are developing for [cross-platform](https://en.wikipedia.org/wiki/Cross-platform_software).  

I want the simplest [web server](https://en.wikipedia.org/wiki/Web_server) ever. It will be used exclusively locally from one super simple [web-application](https://en.wikipedia.org/wiki/Web_application), so  don't need to care much about security. I choose [simple server](https://crates.io/crates/simple-server) from the [rust book](https://doc.rust-lang.org/1.30.0/book/second-edition/ch20-01-single-threaded.html). I don't care about [multi-threading](https://en.wikipedia.org/wiki/Multithreading_(computer_architecture)) or [async](https://en.wikipedia.org/wiki/Asynchrony_(computer_programming)) , because it will be used by only one browser. The example from the book evolved into the github repository of the author of the book [github.com/steveklabnik](https://github.com/steveklabnik/simple-server). I cloned it and update the dependencies and consequently also some broken code from the upgrade.  

For the browser I will create a simple web app. All the code will be in rust, I will avoid javascript as a plague. I just don't like it.  
The GUI will be in [HTML5](https://en.wikipedia.org/wiki/HTML5) and [CSS3](https://en.wikipedia.org/wiki/CSS#CSS_3). This is all supported by all [modern browsers](https://www.bopdesign.com/bop-blog/2012/01/why-use-a-modern-web-browser/).  

## Development

I will use [cargo-auto](https://crates.io/crates/cargo-auto) to automate the tasks needed to build the project. It is a workspace (multi-project), so this will be interesting. I will update the code of cargo-auto to work with workspaces. Cargo-auto is also a project of mine.  
The sub-directory `automation_tasks_rs` is the rust project for [cargo-auto](https://crates.io/crates/cargo-auto).  

The rust workspace is made of members:

- backend CLI (this will be the main and only project to be published on crates.io)
- GUI frontend
- common structs
- simple_server

The sub-directory `web_server_folder` contains all the files and folder structure for a working development web_server.  
But this files are not used directly. Because of the way the publish to crates.io works, I will embed them inside the rust code as strings (base64 encoded if needed). I will make an automation task for that.  

## backend - cargo_crev_reviews

This is a micro-web-server intended for local use with only one local browser connected to it.  
I wanted to give it the name `cargo_crev_reviews_micro_web_server_backend`, but because this one will be published on crates.io, I must give it the name of the whole project `cargo_crev_reviews`.  
It is the backend of the application `cargo_crev_reviews`.  
The frontend is the GUI web app that runs in the browser and is connected only to this backend.  
Together the backend and the frontend form a complete application that is cross-platform.  
They share some structs for communication that are defined in the `common` project.  
The only URL the server operates is: <http://127.0.0.1:8182/cargo_crev_reviews>

If I want to publish this on crates.io it must all be inside one binary executable file.  
It means that all the static files: css, html, icons, images, ... must be inside the rust code.  
For developing it is practical to have all this files as files.  
But before release an automation task converts this files to strings and put them into the rust code.  

The micro-server will accept mostly [POST](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST) with json similar to  [json-rpc](https://www.jsonrpc.org/specification). But sure I had to modify it to something more adequate for my use-case. I think in the future I will change that even more to something more adequate.  

```bash
Syntax:

--> data sent to Server
<-- data sent to Client

rpc call with named parameters:

--> {
"server_method": "subtract_calculate", 
"params": {
    "subtrahend": 23, 
    "minuend": 42, 
    }
}

<-- {
"client_method": "subtract_show", 
"params": {
    "subtracted": 19,     
    }
}
```

An example how to test a POST request with curl:

```bash
curl -d '{"server_method": "subtract", "params": {"subtrahend": 23, "minuend": 42}}' -H 'Content-Type: application/json' http://127.0.0.1:8182/cargo_crev_reviews
```

There are also a small number of GET requests for static files mostly to start the communication between the browser and the server.  

## GUI frontend - cargo_crev_reviews_wasm

This simple web app is the GUI frontend of the application `cargo_crev_reviews`.  
It is strictly designed for use on desktops as it is a tool for programers. No need to do a mobile version.  

## common structs - cargo_crev_reviews_common

Common structures between backend and frontend. It is kind of a contract for communication.  
All in 100% rust language.  
TODO: I have a problem when serialize and deserialize structs. Most of the time I need then name of the field. Because with the name i can bind in different scenarios. Using structs I don't have the name of the field at runtime. I think I will ditch most of the structs to have just a plain old flat text. Inside that text every field will have a slice and a name. And I can then use that in runtime for bindings.  

## simple_server

This is a simple web server. I want to have the code here if I need to modify something or maybe make it more simple.  
The code is from the [rust book](https://doc.rust-lang.org/book/ch20-01-single-threaded.html)
and later from [github.com/steveklabnik](https://github.com/steveklabnik/simple-server) the author of the book.  

## Code-flow

Everything is compiled into one single executable binary for Linux: `cargo_crev_reviews`.  
First it opens the default browser with `xdg-open` on <http://127.0.0.1:8182/cargo_crev_reviews/index.html>.  
If your WSL2 does not have yet a default browser run this:  

``` bash
ln -s "/mnt/c/Program Files/Mozilla Firefox/firefox.exe" /usr/bin/browser_in_win
export BROWSER='/usr/bin/browser_in_win'
```

The command `ln -sf` is permanent and persistent. It makes a symbolic link file that stays there forever. But `export BROWSER=` is NOT persistent. You need to add this command to `~/.bashrc` that runs it on every start of terminal.  

In the next millisecond the web server starts listening to 127.0.0.1 port 8182.  

The first set of requests are GET and response is "static" files embedded in files_mod.rs

1. browser request for `/cargo_crev_reviews/index.html` is GET, the response is html text file embedded in files_mod.rs in the function: `index_html()`  
    This html is just an empty shell that gets the css and wasm code. There is no real content inside. This concept is [Single-page application SPA](https://en.wikipedia.org/wiki/Single-page_application).  
2. index.html requests: 3 css files, `pkg/cargo_crev_reviews.js`, `pkg/cargo_crev_reviews_bg.wasm`, "favicon" `icons/icon-032.png`. All these requests are GET and responses come from files_mod.rs functions, some are text files and others are base64 files.
3. the browser imports the wasm module and starts the init function that requests `review_new.html`. This is GET, the responses is html text files embedded in files_mod. TODO: Maybe this should already be a rpc request
    This is a page with the html content.

4. wasm (inside the browser) is rust code and if needed it modifies the received html and inserts it into index.html
5. the browser renders our first page. Hooray!

6. the user inputs some data and click on the button `review_save`
7. the macro `on_click!` hides the ugly rust code behind the definition of an event handler in web_sys and calls `review_save_on_click()`
8. wasm creates a rpc and sends the POST request to the server
9. the request is POST, the server calls the method `review_save()` and returns a `rpc` response
10. The response contains the html to be rendered and optional data to be inserted in this html before rendering.

## cargo-crev integration

The [cargo-crev](https://github.com/crev-dev/cargo-crev) project contains many crates. The crates `crev-lib` and `crev-data` are libraries for integration.  
I want to create a new simple review, similar to the command:  
`cargo crev crate review -u --skip-activity-check crate_name version`  
The crate and version must already exist inside the local `cargo registry index cache`. That means we use this dependency for some of our projects. We cannot review something we never actually used. Path to an info file: 
`~\.cargo\registry\index\github.com-1ecc6299db9ec823\cache\re\ad\reader_for_microxml`  
The content of this file is roughly:  

```
bc688d353fc7c7a2f3f1f5fed9a27fc1773fc710
1.0.0 {
    "name": "reader_for_microxml",
    "vers": "1.0.0",
    "deps": [],
    "cksum": "623616f68a6441e2f61aa01c9bbcf76f4c9989328e0e10ab747e936718791912",
    "features": {},
    "yanked": true,
    "links": null
}
1.1.11 {
    "name": "reader_for_microxml",
    "vers": "1.1.11",
    "deps": [],
    "cksum": "fd50abb1f0d11a59ebe6d3f31446e4af8d0f8a7df668034b6c9b94453fa30c42",
    "features": {},
    "yanked": true,
    "links": null
}
```

In `~/.cargo/registry/src/github.com-1ecc6299db9ec823/reader_for_microxml-1.1.11/` is the entire source code downloaded from crates.io. `Crates.io` guarantees the source code for a crate-version cannot be altered or even deleted. We know it will never change, so we can review exactly this code with confidence.  

## TODO

crev new review calculate digest and revision
Automation tasks for workspaces:  
auto_lines_of_code: exclude files_mod.rs, because it is just embedded files.  
auto_cargo_toml_to_md, , auto_md_to_doc_comments  
special types for html encoded strings and attribute_value_encoded strings. So I can be sure that I did or did not encoded them.  

## cargo crev reviews and advisory

We leave in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).  
It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
You can also read reviews quickly on the web. Example for the crate `num-traits`:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

## open-source free and free as a beer

My open-source projects are free and free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful,  
please buy me a beer or two donating on my [paypal](https://www.paypal.com/paypalme/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) !

[comment]: # (auto_md_to_doc_comments segment end A)
