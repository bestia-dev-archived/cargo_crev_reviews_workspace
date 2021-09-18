[comment]: # (auto_md_to_doc_comments segment start A)

# dev_bestia_simple_server

[comment]: # (auto_cargo_toml_to_md start)

**simple server from <https://github.com/steveklabnik/simple-server>**  
***[repository](https://github.com/lucianobestia/cargo_crev_reviews_workspace); version: 2021.918.1640  date: 2021-09-18 authors: Luciano Bestia***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-511-green.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-221-blue.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-15-purple.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-96-yellow.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-140-orange.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)

[comment]: # (auto_lines_of_code end)

I want the simplest [web server](https://en.wikipedia.org/wiki/Web_server) ever. It will be used exclusively locally from one super simple [web-application](https://en.wikipedia.org/wiki/Web_application), so  don't need to care much about security. I choose [simple server](https://crates.io/crates/simple-server) from the [rust book](https://doc.rust-lang.org/1.30.0/book/second-edition/ch20-01-single-threaded.html). I don't care about [multi-threading](https://en.wikipedia.org/wiki/Multithreading_(computer_architecture)) or [async](https://en.wikipedia.org/wiki/Asynchrony_(computer_programming)) , because it will be used by only one browser. The example from the book evolved into the github repository of the author of the book [github.com/steveklabnik](https://github.com/steveklabnik/simple-server).  
I cloned it, updated the dependencies and consequently fixed some broken code.  
I had to publish it to crates.io because the project cargo_crev_reviews use it. Crates.io does not accept `local path` dependencies, only crates.io dependencies.

## A simple web-server

The `simple-server` crate is designed to give you the tools to to build
an HTTP server, based around the http crate, blocking I/O, and a
threadpool.

We call it 'simple' want to keep the code small, and easy to
understand. This is why we're only using blocking I/O. Depending on
your needs, you may or may not want to choose another server.
However, just the simple stuff is often enough for many projects.

## Examples

At its core, `simple-server` contains a `Server`. The `Server` is
passed a handler upon creation, and the `listen` method is used
to start handling connections.

The other types are from the `http` crate, and give you the ability
to work with various aspects of HTTP. The `Request`, `Response`, and
`ResponseBuilder` types are used by the handler you give to `Server`,
for example.

To see examples of this crate in use, please consult the `examples`
directory.

[comment]: # (auto_md_to_doc_comments segment end A)
