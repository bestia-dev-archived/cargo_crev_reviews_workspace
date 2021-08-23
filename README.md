[comment]: # (auto_md_to_doc_comments segment start A)

# cargo_crev_reviews

[comment]: # (auto_cargo_toml_to_md start)

Write cargo-crev reviews in a Graphical User Interface

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)

[comment]: # (auto_lines_of_code end)

## Motivation

I think [cargo-crev](https://lib.rs/crates/cargo-crev) is a great tool to express trustworthiness in the open-source community, especialy for the [rust programming language](https://www.rust-lang.org/).  I fear so much of [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack) using dependencies from [crates.io](https://crates.io/). For the smallest project you can get 100 dependencies easily. How to trust them all? To review them all manually? It is just crazy.  
But if enought people write reviews, it will be so much easier to trust the code. It is the same principle as [booking.com](https://www.booking.com/) or [air-bnb](https://www.airbnb.com/). Guests of a hotel write a review about their actual experience in the hotel. And you can read a hopefully truthfull review and can understand if the hotel is good or bad. Sometimes can happen to find a fake review, but if there is enough people, most of them will be sincere.  
Sadly, writing reviews in `cargo-crev` is very hard if you are not a die-hard [VIM](https://www.vim.org/) user. I just cannot do that. I am a [VSCode](https://code.visualstudio.com/) guy. I don't intend to lear VIM. Sorry.  
This is a great project for me to play and learn with [rust](https://www.rust-lang.org/): let's make a [GUI](https://en.wikipedia.org/wiki/Graphical_user_interface) to write reviews for `cargo-crev`.  
And we will see walking the path what obstacles we must overcome.  

## Technical decisions

Rust does not have a true GUI story. It is mostly for [CLI](https://en.wikipedia.org/wiki/Command-line_interface) and libraries.  
But it is the best language for [Wasm/Webassembly](https://webassembly.org/). So let's combine this.  
I will make a rust [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) (multi-projects):

1. CLI for a web server (microserver)
2. Wasm for the browser (chrome and similar)

The web server CLI will access files, commands, libraries and maybe the network. This will work only in [Linux](https://en.wikipedia.org/wiki/Linux), but today Win10 has integrated Linux with [WSL2](https://en.wikipedia.org/wiki/Windows_Subsystem_for_Linux). It will work just fine on most common existing operating systems.  
Wasm in browser will just access to our micro web server. This is gonna be our GUI and because browser work in every [OS](https://en.wikipedia.org/wiki/Operating_system), we are developing for [cross-platform](https://en.wikipedia.org/wiki/Cross-platform_software).  

I want the simplest [web server](https://en.wikipedia.org/wiki/Web_server) ever. It will be used exclusivelly locally from one super simple [web-application](https://en.wikipedia.org/wiki/Web_application), so  don't need to care much about security. I choose [simple server](https://crates.io/crates/simple-server) from the [rust book](https://doc.rust-lang.org/1.30.0/book/second-edition/ch20-01-single-threaded.html). I don't care about [multi-threading](https://en.wikipedia.org/wiki/Multithreading_(computer_architecture)) or [async](https://en.wikipedia.org/wiki/Asynchrony_(computer_programming)) , because it will be used by only one browser.  

For the browser I will create a [PWA - Progressive Web Application](https://en.wikipedia.org/wiki/Progressive_web_application). All the code will be in rust, I will avoid javascript as a plague. I just don't like it. The GUI will be in [HTML5](https://en.wikipedia.org/wiki/HTML5) and [CSS3](https://en.wikipedia.org/wiki/CSS#CSS_3). This is all supported by all [modern browsers](https://www.bopdesign.com/bop-blog/2012/01/why-use-a-modern-web-browser/). I will use my project [rust_wasm_helper_for_pwa](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa) that creates a minimal PWA you can copy to your project.  

## Development

I will use [cargo-auto](https://crates.io/crates/cargo-auto) to automate the tasks needed to build the project. It is a workspace (multi-project), so this will be interesting. Probably a new template for my [cargo-auto](https://crates.io/crates/cargo-auto) will be born from this project.  



## cargo crev reviews and advisory

We leave in times of danger with `supply chain attacks`.  
It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
You can also read reviews quickly on the web. Example for the crate `num-traits`:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

## open-source free and free as a beer

My open-source projects are free and free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer or two donating on my [paypal](https://www.paypal.com/paypalme/LucianoBestia). You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) !

[comment]: # (auto_md_to_doc_comments segment end A)
