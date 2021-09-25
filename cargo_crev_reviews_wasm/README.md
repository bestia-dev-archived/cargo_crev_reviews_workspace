[comment]: # (auto_md_to_doc_comments segment start A)

# cargo_crev_reviews_wasm

[comment]: # (auto_cargo_toml_to_md start)

**Wasm web app that is the frontend of the application cargo_crev_reviews**  
***[repository](https://github.com/LucianoBestia/cargo_crev_reviews_workspace); version: 2021.925.1120  date: 2021-09-25 authors: Luciano Bestia***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1281-green.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-147-blue.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-104-purple.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)

[comment]: # (auto_lines_of_code end)

This is a member of the workspace.
It is the wasm frontend for the GUI application.  

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
3. the browser imports the wasm module and starts the init function that requests `srv_review_list`. This responds with: response_method_name, response_html and response_data.
4. wasm (inside the browser) is rust code. First it matches method_name and calls the appropriate function. It processes the html with the data and inserts it into index.html (the empty shell).
5. the browser renders our first page. Hooray!
6. the user click on some button.
7. the macro `on_click!` or `row_on_click!` hides the ugly rust code behind the definition of an event handler in web_sys and calls a function
8. wasm creates a rpc request and sends/POST to the server
9. the request is POST, the server first matches the method_name and calls the appropriate function. The function processes the call and prepares some data. It loads the html template.
10. The response contains the html to be rendered and data to be inserted in this html before rendering.

[comment]: # (auto_md_to_doc_comments segment end A)
