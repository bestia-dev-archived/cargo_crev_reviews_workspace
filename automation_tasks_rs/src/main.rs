//! automation_tasks_rs workspace

use cargo_auto_lib::*;
use unwrap::unwrap;

/// automation_tasks_rs workspace
fn main() {
    exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion. Take care to keep them in sync with the changes.

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("Running automation task: {}", &task);
                if &task == "build" || &task == "b" {
                    task_build();
                } else if &task == "release" || &task == "r" {
                    task_release();
                } else if &task == "copy_common" {
                    task_copy_common();
                } else if &task == "docs" || &task == "doc" || &task == "d" {
                    task_docs();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "build_and_run" {
                    task_build_and_run();
                } else if &task == "release_and_run" {
                    task_release_and_run();
                } else {
                    println!("Task {} is unknown.", &task);
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
User defined tasks in automation_tasks_rs:
cargo auto build - builds the crate in debug mode, fmt
cargo auto build_and_run - build and run
cargo auto release - builds the crate in release mode, version from date, fmt
cargo auto release_and_run - release and run
cargo auto copy_common - copy common_mod.rs from cargo_crev_reviews to cargo_crev_reviews_wasm
cargo auto docs - builds the docs, copy to docs directory
cargo auto commit_and_push - commits with message and push with mandatory message
    if you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git
cargo auto publish_to_crates_io - publish to crates.io, git tag
"#
    );
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "build_and_run", "release", "release_and_run","copy_common", "doc", "commit_and_push"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }

    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["with_lib"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// build every member of workspace. One is wasm project, so instead of cargo build, I use wam-pack build
/// TODO: if the member ends with "_wasm" then exclude from `cargo build` and build with `wasm-pack build` first
/// for faster build I will change only the version number to members that was modified
fn task_build() {
    auto_check_micro_xml("web_server_folder/cargo_crev_reviews");
    task_copy_common();
    auto_version_from_date();
    run_shell_command("cargo fmt");
    run_shell_command("cd cargo_crev_reviews_wasm;wasm-pack build --target web;cd ..");
    // copy to web_server_folder/pkg
    run_shell_command("rsync -a --info=progress2 --delete-after cargo_crev_reviews_wasm/pkg/ web_server_folder/cargo_crev_reviews/pkg/");
    
    copy_web_folder_files_into_module();
    run_shell_command("cargo build --workspace --exclude cargo_crev_reviews_wasm");

    println!(
        r#"
After `cargo auto build`, run the tests and the code. If ok, then 
run `cargo auto release`
"#
    );
}

/// build release every member of workspace. One is wasm project, so instead of cargo build, I use wam-pack build
/// TODO: if the member ends with "_wasm" then exclude from `cargo build` and build with `wasm-pack build` first
/// this workspace is basically one single application splitted into 3 projects
/// it deserves the same version number for the release build. It means that it will build all members. 
/// A little slower than only build.
fn task_release() {
    auto_check_micro_xml("web_server_folder/cargo_crev_reviews");
    task_copy_common();
    auto_version_from_date_forced();    
    run_shell_command("cargo fmt");

    run_shell_command("cd cargo_crev_reviews_wasm;wasm-pack build --target web --release;cd ..");
    // copy to web_server_folder/pkg
    run_shell_command("rsync -a --info=progress2 --delete-after cargo_crev_reviews_wasm/pkg/ web_server_folder/cargo_crev_reviews/pkg/");

    //auto_cargo_toml_to_md();

    // I need to exclude the file `cargo_crev_reviews/src/files_mod.rs` because it contains only emendable files
    // I will empty this files, and later it will be filled with copy_web_folder_files_into_module
    unwrap!(std::fs::write("cargo_crev_reviews/src/files_mod.rs", ""));
    auto_lines_of_code("");
    copy_web_folder_files_into_module();


    run_shell_command("cargo build --release --workspace --exclude cargo_crev_reviews_wasm");
    run_shell_command(&format!("strip target/release/{}", package_name()));

    println!(
        r#"
After `cargo auto release`, run the code. If ok, then 
run `cargo auto doc`
"#
    );
}

/// after release, run the web server and it will automatically open the browser
fn task_build_and_run() {
    task_build();
    run_shell_command("target/debug/cargo_crev_reviews");
    println!(
        r#"
After `cargo auto build_and_run` close the CLI with ctrl+c and close the browser.
"#
    );
}

/// after release, run the web server and it will automatically open the browser
fn task_release_and_run() {
    task_release();
    run_shell_command("target/release/cargo_crev_reviews");
    println!(
        r#"
After `cargo auto release_and_run` close the CLI with ctrl+c and close the browser.
"#
    );
}

fn task_copy_common() {
    std::fs::copy("cargo_crev_reviews/src/common_mod.rs", "cargo_crev_reviews_wasm/src/common_mod.rs").unwrap();
}

/// example how to call a list of shell commands and combine with rust code
fn task_docs() {
    // auto_md_to_doc_comments();
    #[rustfmt::skip]
    let shell_commands = [
        "cargo doc --no-deps --document-private-items --open",
        // copy target/doc into docs/ because it is github standard
        "rsync -a --info=progress2 --delete-after target/doc/ docs/",
        "echo Create simple index.html file in docs directory",
        &format!("echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",package_name().replace("-","_")) ,
    ];
    run_shell_commands(shell_commands.to_vec());
    // message to help user with next task
    println!(
        r#"
After `cargo auto doc`, check `docs/index.html`. If ok, then 
run `cargo auto commit_and_push` with mandatory commit message
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    match arg_2 {
        None => println!("Error: message for commit is mandatory"),
        Some(message) => {
            run_shell_command(&format!(r#"git add -A && git commit -m "{}""#, message));
            run_shell_command("git push");
            println!(
                r#"
After `cargo auto commit and push`
run `cargo auto publish_to_crates_io`
"#
            );
        }
    }
}

// endregion: tasks

/// copy all files in the web_server_folder as strings to the module `files_mod.rs`
fn copy_web_folder_files_into_module() {
    /// read all files and push rust code into module
    /// nested function or inner function, cannot capture environment as closures. Good.
    fn copy_files_from_dir(root_directory: &str, module_source_code: &mut String) {
        let path = std::path::Path::new(root_directory);
        for entry in unwrap!(path.read_dir()) {
            if let Ok(entry) = entry {
                let p: std::path::PathBuf = entry.path();
                if p.is_file() {
                    let ps = p.to_string_lossy();
                    if !ps.ends_with(".gitignore")
                        && !ps.ends_with("icon-original.png")
                        && !ps.ends_with("README.md")
                        && !ps.ends_with("package.json")
                        && !ps.ends_with("cargo_crev_reviews_wasm_bg.wasm.d.ts")
                        && !ps.ends_with("cargo_crev_reviews_wasm.d.ts")
                        && !ps.ends_with("LICENSE")
                    {
                        let start = format!(
                            "\npub fn {}() -> &'static str{{\nr##\"\n",
                            ps.trim_start_matches("web_server_folder/cargo_crev_reviews/")
                                .replace("/", "_")
                                .replace(".", "_")
                                .replace("-", "_")
                                .to_lowercase()
                        );
                        module_source_code.push_str(&start);

                        // binary files are encoded base64
                        let body = if ps.ends_with(".png") || ps.ends_with(".woff2") || ps.ends_with(".wasm") {
                            let e = base64::encode(unwrap!(std::fs::read(p)));
                            // it is much easier to have lines of 76 characters in rust source code.
                            // before decoding base64 I will eliminate \n
                            // rust string is utf8, but base64 is strictly ascii.
                            // So I have the guarantee 100% that 1 byte = 1 char
                            let multi_line = e.as_bytes().chunks(76).collect::<Vec<_>>().join(&b'\n');
                            unwrap!(String::from_utf8(multi_line))
                        } else {
                            unwrap!(std::fs::read_to_string(p))
                        };
                        module_source_code.push_str(&body);

                        let end = format!("\n\"##\n}}\n");
                        module_source_code.push_str(&end);
                    }
                }
            }
        }
    }

    let mut module_source_code = String::new();
    module_source_code.push_str("// files_mod.rs\n\n");
    copy_files_from_dir("web_server_folder/cargo_crev_reviews", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/css", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/icons", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/js", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/pkg", &mut module_source_code);
    unwrap!(std::fs::write("cargo_crev_reviews/src/files_mod.rs", module_source_code));
}
