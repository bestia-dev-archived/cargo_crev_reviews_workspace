[comment]: # (auto_md_to_doc_comments segment start A)

# cargo_crev_reviews

[comment]: # (auto_cargo_toml_to_md start)

**Write cargo-crev reviews in GUI with a cross-platform app written in full-stack Rust**  
***[repository](https://github.com/lucianobestia/cargo_crev_reviews_workspace); version: 2022.125.1258  date: 2022-01-25 authors: Luciano Bestia***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-2948-green.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-556-blue.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-242-purple.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-27-orange.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)

[comment]: # (auto_lines_of_code end)

[comment]: # (auto_badges start)

[![crates.io](https://img.shields.io/crates/v/cargo_crev_reviews.svg)](https://crates.io/crates/cargo_crev_reviews) [![Documentation](https://docs.rs/cargo_crev_reviews/badge.svg)](https://docs.rs/cargo_crev_reviews/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo_crev_reviews.svg)](https://web.crev.dev/rust-reviews/crate/cargo_crev_reviews/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo_crev_reviews/) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/cargo_crev_reviews/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/cargo_crev_reviews/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/cargo_crev_reviews/actions)  

[comment]: # (auto_badges end)

## Try it

Install cargo_crev_reviews:  

```bash
cargo install cargo_crev_reviews
```

Go to a Rust project directory where the Cargo.toml file is, and start the program:  

```bash
cd ~/rustprojects/your-project-name
cargo_crev_reviews
```

And follow the simple instructions...

Backend CLI in Linux terminal:  
![screen_5](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/raw/main/images/screen_5.png "screen_5")  
Frontend GUI in browser:  
![screen_6](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/raw/main/images/screen_6.png "screen_6")  

## cargo tree

Cargo-tree is a Rust utility that shows all the ramification of dependencies in your Rust project. It is included inside the cargo utility.

## your personal reviews

Your personal reviews are the most important. Ideally, you want to personally review every crate, rate it and write something about it for your own use. You want to know that the dependencies your program is using are not malicious or unsound. If you have a boss, he will sooner or later ask you if you reviewed all the dependencies. With `cargo_crev_reviews` you have a basic tool to do that.  

Don't panic!  

To write a review you need to see the exact source code and other metadata about the crate. Click on the crate name in the list and it will open:

- VSCode in the directory with a copy of the original dependency source code
- [crev.dev](https://web.crev.dev/rust-reviews/crates/) for other people reviews
- crates.io for basic information
- lib.rs for extended information
- a list of all versions of that crate with your reviews added

Be warned that modern browsers block pop-ups and you have to allow that explicitly for this site `127.0.0.1`.  
If you don't have VSCode, you can change the code_editor in the `Config` menu.  

## Reputation vs. code review

Personally, I think that the reputation of the author is important. For some highly visible and respected members of the Rust community I don't review the code. The reputation of the author is enough to make me feel safe. This method is not perfect, because there can be identity theft or a faulty version. But I still think that it is an efficient and effective method for me.  
Crates.io made a confusion with authors and owners. Lately they introduced the `published_by` field for every published version. For me this is the main person responsible for any eventual issue of this specific published version. Crates.io guarantees that published versions cannot be modified later. They are read-only, so you know that you see exactly the right code.  

## Share it with the community

When you are satisfied with your own reviews, you then publish them for other developers to read. In the same way, reviews from other developers help you when analyzing the dependencies. More people that write reviews, better the information we get for our decisions.  
One crate can have many versions. Every version is separately reviewed. Often, the review will be just the same for many versions, but that is good. When a developer wants to see the reviews for a specific version, he does not need to watch the reviews of all other versions of the same crate.

## TL;DR

For developers that want to understand, tweak the code and contribute to the project, below is a long description of the important aspects of the project. If you are only a user of `cargo_crev_reviews`, you don't need to read all of this.  

## Motivation

I think [cargo-crev](https://lib.rs/crates/cargo-crev) is a great tool to express trustworthiness in the open-source community, especially for the [Rust programming language](https://www.rust-lang.org/).  I fear so much of [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack) using dependencies from [crates.io](https://crates.io/). For the smallest project you can get 100 dependencies easily. How to trust them all? To review them all manually? It is just crazy.  
But if enough people write reviews, it will be so much easier to trust the code. It is the same principle as [booking.com](https://www.booking.com/) or [air-bnb](https://www.airbnb.com/). Guests of a hotel write a review about their actual experience in the hotel. And you can read a hopefully truthful review and can understand if the hotel is good or bad. Sometimes can happen to find a fake review, but if there is enough people, most of them will be sincere.  
Sadly, writing reviews in `cargo-crev` is hard. Let's make a [GUI](https://en.wikipedia.org/wiki/Graphical_user_interface) wrapper around `cargo-crev` to make write reviews easier.  
We will see walking the path what obstacles we must overcome.  

## Technical decisions

Rust does not have a true GUI story. It is mostly for [CLI](https://en.wikipedia.org/wiki/Command-line_interface) and libraries. Because GUI is mostly non cross-platform. But Rust is the best language for [Wasm/Webassembly](https://webassembly.org/). So let's combine this.  
I will make a Rust [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) (multi-projects):

1. CLI for a web server (local micro-server)
2. Wasm for the browser (chrome and similar)

The web server CLI will access files, commands, libraries and the network. This will work only in [Linux](https://en.wikipedia.org/wiki/Linux), but today Win10 has integrated Linux with [WSL2](https://en.wikipedia.org/wiki/Windows_Subsystem_for_Linux). It will work just fine on all the operating systems sensible for Rust development.  
Wasm in browser will just access the local micro web server. This is gonna be the GUI and because the browser works in every [OS](https://en.wikipedia.org/wiki/Operating_system), this is [cross-platform](https://en.wikipedia.org/wiki/Cross-platform_software) development.  

I want the simplest [web server](https://en.wikipedia.org/wiki/Web_server) ever. It will be used exclusively locally from one super simple [web-application](https://en.wikipedia.org/wiki/Web_application), so  don't need to care much about security. I choose [simple server](https://crates.io/crates/simple-server) from the [Rust book](https://doc.rust-lang.org/1.30.0/book/second-edition/ch20-01-single-threaded.html). I don't care about [multi-threading](https://en.wikipedia.org/wiki/Multithreading_(computer_architecture)) or [async](https://en.wikipedia.org/wiki/Asynchrony_(computer_programming)) , because it will be used by only one browser. The example from the book evolved into the github repository of the author of the book [github.com/steveklabnik](https://github.com/steveklabnik/simple-server). I cloned it and update the dependencies and consequently fixed some broken code. I published it as dev_bestia_simple_server on crates.io.  

For the browser I will create a simple web app. All the code will be in Rust, I will avoid javascript. The GUI will be in [HTML5](https://en.wikipedia.org/wiki/HTML5) and [CSS3](https://en.wikipedia.org/wiki/CSS#CSS_3). This is all supported by all [modern browsers](https://www.bopdesign.com/bop-blog/2012/01/why-use-a-modern-web-browser/).  

## Development

I will use [cargo-auto](https://crates.io/crates/cargo-auto) to automate the tasks needed to build the project.  
The sub-directory `automation_tasks_rs` is the Rust project for [cargo-auto](https://crates.io/crates/cargo-auto).  
My project [dev_bestia_cargo_completion](https://github.com/LucianoBestia/dev_bestia_cargo_completion) helps with bash auto-completion.  

The Rust workspace is made of members:

- backend CLI (this will be the main and only project to be published on crates.io)
- GUI frontend

The sub-directory `web_server_folder` contains all the files and folder structure for a working development web_server.  
But this files are not used directly. Because of the way the publish to crates.io works, I will embed them inside the Rust code as strings (base64 encoded if needed). I will make an automation task for that.  

There is a file `auto_generated_mod.rs` where automation tasks generates boilerplate code. I prefer generated code to `procedural macros` because it is easy to write(generate), read and debug the code. Also the tools for auto-completion can have problems with procedural macros.  

## backend - cargo_crev_reviews

This is a micro-web-server intended for local use with only one local browser connected to it.  
It is the backend of the application `cargo_crev_reviews`. I had to use the same name here.  
Together the backend and the frontend form a complete application that is cross-platform.  
They share some structs for communication that are defined in the `common_structs_mod` module. One automation task copies the content from backend to frontend projects to keep them in sync.  
The only URL the server operates is: <http://127.0.0.1:8182/cargo_crev_reviews>

The web server CLI will access files, commands, libraries and the network.  For the signing process of crev reviews it need the crev passphrase. This can be entered interactively or with the env variable `_export CREV_PASSPHRASE=your_passphrase`.  
Add a space before the export command to avoid the secret to be saved in the bash history.  

If I want to publish this on crates.io it must all be inside one binary executable file.  
It means that all the static files: css, html, icons, images, ... must be inside the Rust code.  
For developing it is practical to have all this files as files.  
But before release an automation task converts this files to strings and put them into the Rust code.  

The micro-server will accept mostly [POST](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST) with json similar to  [json-rpc](https://www.jsonrpc.org/specification). But sure I had to modify it to something more adequate for my use-case. I think in the future I will change that even more to something even more adequate.  

```bash
Syntax:

--> data sent to Server - request
<-- data sent to Client - response

rpc call with named parameters:

--> {
"request_method": "subtract_calculate", 
"request_data": {
    "subtrahend": 23, 
    "minuend": 42, 
    }
}

<-- {
"response_method": "subtract_show", 
"response_data": {
    "subtracted": 19,     
    },
"response_html":"..."    
}
```

An example how to test a POST request with curl:

```bash
curl -d '{"request_method": "subtract", "request_data": {"subtrahend": 23, "minuend": 42}}' -H 'Content-Type: application/json' http://127.0.0.1:8182/cargo_crev_reviews
```

There are also a small number of GET requests for static files mostly to start the communication between the browser and the server.  
TODO: one day I will add also a websocket communication, so the client can show a progress bar for actions processing on the server.

## GUI frontend - cargo_crev_reviews_wasm

This simple web app is the GUI frontend of the application `cargo_crev_reviews`.  
It is strictly designed for use on desktops as it is a tool for programers. No need to do a mobile version. The response from the rpc server contains:

1. response method name
2. html template
3. data

The method name is used to match and call the appropriate function.  
The html template must be microXml compatible. The wasm code reads element by element and when finds a marker, inserts the data. I wanted the html template to be complete with some sample texts. So the markers are added in front of the element or attribute they are meant to replace.  

## common structs - common_structs_mod.rs

Common structures between backend and frontend. It is kind of a contract for communication.  
All in 100% Rust language. One automation task keeps in sync the backend and frontend module.  
TODO: Using structs is not generic enough. Most of the time I need then name of a field. Because with the name I can bind in different scenarios. Using structs I don't have the name of the field at runtime. I think I will ditch most of the structs to have just a plain old flat text with QVS21. Inside that text every field will have a slice and a name. And I can then use that in runtime for bindings.  

## cargo-crev integration

The [cargo-crev](https://github.com/crev-dev/cargo-crev) project contains many crates. The crates `crev-lib` and `crev-data` are libraries for integration. All the code working with crev is encapsulated in the crev_mod.rs module.  

## cargo registry

The cargo application is essential for work with the Rust language. Cargo maintains a local `cargo registry` in the directory `~/.cargo/registry/`. It has 3 basic sub-directories: index, cache and src.  
The `registry index` is the database with crates metadata and is git fetched from github. Path to an index file:  
`~/.cargo/registry/index/github.com-1ecc6299db9ec823/cache/re/ad/reader_for_microxml`  

The content of this file looks like this:  

```yaml
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

Cargo downloads from crates.io the complete source code for every dependency it needs for your project.  
First it downloads the tar gz file ending with `.crate` into the cache directory:  
`~/.cargo/registry/cache/github.com-1ecc6299db9ec823/cargo_auto_lib-0.7.23.crate`  
Probably the `cksum` field in the `registry index` is calculated from this `.crate` file.  
Then this is unpacked into the `src` folder as the complete source code directories and files:  
`~/.cargo/registry/src/github.com-1ecc6299db9ec823/`.  
`Crates.io` guarantees the `.crate` file for a crate+version cannot be altered or deleted and are always available for download from crates.io (even when yanked).  
We can review exactly this local code with confidence, because we know it will never change.  
This local files should not be altered in any way. But it can happen unknowingly and unwillingly, if we open a code editor with auto-complete in this folder. It will create the `target` folder and `Cargo.lock` file. It can happen also that `Go to definition` opens this files in the editor and maybe we alter some comment or code. This is no good.  
In the `config and utils` it is possible to do an integrity check that these files are not tempered. If something is modified it is easy to just remove that folder from `src`. The next time cargo needs these crates it will download and unpack automatically.

## crates.io API

Some data are not available locally in the cargo registry and need to be obtained from <https://crates.io//api/v1/crates/{}/{}/>. Then I store them in `~/.config/crev/cargo_crev_reviews_data/db`. Data from crates.io are immutable all, except yanked. I will find in the local registry the data for yanked and new versions. That will trigger the download of data from crates.io.

## RustSec cargo audit

I added the result of `cargo audit --json` into the cargo tree list.  

## trusted publishers

There is a confusion on crates.io who is the owner, author or group that is responsible for a crate version. Lately they added a `published_by` field for a crate_version. That sounds more accurate. The one that published is responsible to check there is no malware inside.  
You can edit your list of Trusted publishers in the app.  

## Code-flow

Everything is compiled into one single executable binary for Linux: `cargo_crev_reviews`.  
First it opens the default browser with `xdg-open` on <http://127.0.0.1:8182/cargo_crev_reviews/index.html>.  
I received a comment that `xdg-open` is not preinstalled on every Linux distro. I use Debian 10 and have it.  
On other distros it is possible to install it with `xdg-utils`.  
You can also change it with the env variable `export CREV_BROWSER_PATH=/usr/bin/xdg-open` for the command that exists on your system. Later you can change this command in the Config menu.  
If your WSL2 does not have yet a default browser run this:  

``` bash
ln -s "/mnt/c/Program Files/Mozilla Firefox/firefox.exe" /usr/bin/browser_in_win
export BROWSER='/usr/bin/browser_in_win'
```

The command `ln -sf` is permanent and persistent. It makes a symbolic link file that stays there forever.  
But `export BROWSER=` is NOT persistent. You need to add this command to `~/.bashrc` that runs it on every start of terminal.  

In the next millisecond the web server starts listening to `127.0.0.1` port `8182`.  
The first set of requests are GET and response is "static" files embedded in auto_generated_files_mod.rs

1. browser request for `/cargo_crev_reviews/index.html` is GET, the response is html text file embedded in auto_generated_files_mod.rs in the function: `index_html()`  
    This html is just an empty shell that gets the css and wasm code. There is no real content inside. This concept is [Single-page application SPA](https://en.wikipedia.org/wiki/Single-page_application).  
2. index.html requests: 3 css files, `pkg/cargo_crev_reviews.js`, `pkg/cargo_crev_reviews_bg.wasm`, "favicon" `icons/icon-032.png`. All these requests are GET and responses come from auto_generated_files_mod.rs functions, some are text files and others are base64 files.
3. the browser imports the wasm module and starts the init function that requests `request_cargo_tree_list`. This responds with: response_method_name, response_html and response_data.
4. wasm (inside the browser) is Rust code. First it matches method_name and calls the appropriate function. It processes the html with the data and inserts it into index.html (the empty shell).
5. the browser renders our first page. Hooray!
6. the user clicks on some button.
7. the macro `on_click!` or `row_on_click!` hides the ugly Rust code behind the definition of an event handler in web_sys and calls a function
8. wasm creates a rpc request and sends/POST to the server
9. the request is POST, the server first matches the method_name and calls the appropriate function. The function processes the call and prepares some data. It loads the html template.
10. The response contains the html to be rendered and data to be inserted in this html before rendering.

## How to add a page with data

### Define the data structs

In `common_structs_mod.rs` add the structs like:

``` bash
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VersionItemData {
    pub crate_name: String,
    pub crate_version: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VersionListData {
    pub list_of_version: Vec<VersionItemData>,
}
```

### Create a "standard" html page

The html page has to be MicroXml compatible, basically XHtml.  Copy for example `web_server_folder/review_list.html` to a new html file. Open this file with a browser to preview it. I use the VSCode extension [vscode-open-wsl](https://marketplace.visualstudio.com/items?itemName=NoThlnG.vscode-open-wsl) and right-click on the file and `Open with default application`. In WSL2 I use my project [wsl_open_browser](https://github.com/LucianoBestia/wsl_open_browser) to open the default browser from wsl2 to Windows. Now edit the html file to your liking and reload the browser with F5 to see the changes. Use some sample text to make it look as close to what you want. These texts will be later programmatically replaced, but they are invaluable while designing a nice layout.

### Add markers

Inside the html you want to replace the sample texts with the data from the server. Before the text add the (invisible) marker for example `<!--wt_crate_name-->`. You can replace also an attribute if you insert an attribute before it like this `data-wt_variable_name="next_attribute_name"`.  
Now run the automation task `cargo auto build` that will copy/embed this file into `auto_generated_files_mod.rs`.  

### Write a server functions

In `srv_methods_mod.rs` add a function like this:  

```ignore
#[named]
pub fn srv_function_name(request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data));
    let response_data  = get_some_data(filter)?;
    let response_html = crate::auto_generated_files_mod::review_edit_html();
    // cln_methods::cln_review_edit(response_data, response_html)
}
```

Now run the automation task `cargo auto build` that generates `auto_generated_mod.rs`.

### client module

If I use a different struct for data, I must have different client modules and functions.  
With a generic data-struct all of this could be generic.  
TODO: use QVS21

In `cln_version_mod.rs` add a client function like this:

```ignore
pub fn cln_review_edit(srv_response: RpcResponse) {
    let html = extract_html(&srv_response);
    store_to_review_item_data(srv_response);

    // the mutex is locked inside a scope. When this structure falls out of scope, the lock will be unlocked.
    let html_after_process = {
        let data = REVIEW_ITEM_DATA.lock().unwrap().deref();
        process_html(data, &html)
    };

    inject_into_html(&html_after_process);

    on_click!("button_review_save", request_review_save);
    on_click!("button_review_list", request_review_list);
}
```

## sled database

I don't want to repeatedly use crates.io api for the same data. I need a disk persistent storage for this data.  
I will have a try with the [sled](http://sled.rs/) database. A lightweight pure-Rust high-performance transactional embedded database. This is a key-value database. The value can be any struct. There can be multiple separate trees/keyspaces: crates, versions, reviews, yanked,....  

## plantUml

Update 2022: I now try to use `mermaid` instead of plantuml, because it is integrated in the Github markdown.

Write your diagrams in code with plantUml. The language syntax is pretty easy. I have a text file `images/server_plantuml_v3.txt`. The I use the online service `plantuml.com/plantuml/proxy` to create a png image. Because of caching I have the version of the file in the file name. When the version changes the old cache is not in use any more.  
The server modules:

[![plant_uml](https://www.plantuml.com/plantuml/proxy?src=https://raw.githubusercontent.com/LucianoBestia/cargo_crev_reviews_workspace/main/images/server_plantuml_v3.txt)](https://raw.githubusercontent.com/LucianoBestia/cargo_crev_reviews_workspace/main/images/server_plantuml_v3.txt)

## Bad surprise on WSL2

My git repository got corrupted. It looks that this happens to many people on WSL2.  
The cure is:  

```batch
# backup the repo first!
find .git/objects/ -type f -empty | xargs rm
git fetch -p
git fsck --full
```

## Some other info

Cargo-crev knows also about Issues and Advisories. But for this project I focused only on Reviews. You can rate the version as Negative if there is something really broken with it.
I also limited the crates source to only `crates.io`. Other sources are not publicly interesting.

## cargo crev reviews and advisory

We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).  
It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
You can also read reviews quickly on the web:  
<https://web.crev.dev/rust-reviews/crates/>  

## open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful,  
please buy me a beer donating on my [paypal](https://www.paypal.com/paypalme/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[comment]: # (auto_md_to_doc_comments segment end A)
