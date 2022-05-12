//! automation_tasks_rs workspace

use cargo_auto_lib::*;
use unwrap::unwrap;

mod utils_mod;
mod cargo_crev_reviews_special_mod;
use cargo_crev_reviews_special_mod::*;

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
                if &task == "build" {
                    task_build();
                } else if &task == "build_and_run" {
                    task_build_and_run();
                } else if &task == "release" {
                    task_release();
                } else if &task == "release_and_run" {
                    task_release_and_run();
                } else if &task == "test" {
                    task_test();
                } else if &task == "doc" {
                    task_docs();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "publish_to_crates_io_cargo_crev_reviews" {
                    task_publish_to_crates_io_cargo_crev_reviews();
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
cargo auto docs - plantuml, builds the docs, copy to docs directory
cargo auto test - runs all the tests
cargo auto commit_and_push - commits and push with mandatory message
    if you use SSH for git push, it is easy to start the `eval $(ssh-agent)` in the background 
    and your credentials for git with `ssh-add ~/.ssh/your_private_key`
cargo auto publish_to_crates_io_cargo_crev_reviews - publish to crates.io, git tag
"#
    );
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "build_and_run", "release", "release_and_run", "doc", "test", "commit_and_push","publish_to_crates_io_cargo_crev_reviews"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }

    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
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
    task_generated_mod();
    auto_version_increment_semver_or_date();
    run_shell_command("cargo fmt");
    run_shell_command("cd cargo_crev_reviews_wasm;wasm-pack build --target web;cd ..");
    // copy to web_server_folder/pkg
    run_shell_command("rsync -a --info=progress2 --delete-after cargo_crev_reviews_wasm/pkg/ web_server_folder/cargo_crev_reviews/pkg/");
    
    copy_web_folder_files_into_module();
    run_shell_command("cargo build --workspace --exclude cargo_crev_reviews_wasm");

    println!(
        r#"
After `cargo auto build`, 
run `cargo auto build_and_run` or
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
    task_generated_mod();
    auto_version_increment_semver_or_date_forced();    
    run_shell_command("cargo fmt");

    run_shell_command("cd cargo_crev_reviews_wasm;wasm-pack build --target web --release;cd ..");
    // copy to web_server_folder/pkg
    run_shell_command("rsync -a --info=progress2 --delete-after cargo_crev_reviews_wasm/pkg/ web_server_folder/cargo_crev_reviews/pkg/");

    auto_cargo_toml_to_md();

    auto_lines_of_code("");
    copy_web_folder_files_into_module();

    run_shell_command("cargo build --release --workspace --exclude cargo_crev_reviews_wasm");
    let cargo_toml = CargoToml::read();
    run_shell_command(&format!("strip target/release/{}", cargo_toml.package_name()));

    println!(
        r#"
After `cargo auto release`, 
run `cargo auto release_and_run` or
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

/// modify auto_generated_mod.rs
fn task_generated_mod() {
    common_structs_copy();
    generate_server_methods();
    generate_client_match_response_method();
    generate_client_methods();
    generate_server_match_response_method();
}

/// cargo doc
fn task_docs() {
    let cargo_toml = CargoToml::read();
    auto_md_to_doc_comments();
    auto_plantuml(&cargo_toml.package_repository().unwrap());

    #[rustfmt::skip]
    let shell_commands = [
        "cargo doc --no-deps --document-private-items",
        // copy target/doc into docs/ because it is github standard
        "rsync -a --info=progress2 --delete-after target/doc/ docs/",
        "echo Create simple index.html file in docs directory",
        &format!("echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",cargo_toml.package_name().replace("-","_")) ,
    ];
    run_shell_commands(shell_commands.to_vec());
    run_shell_command("cargo fmt");
    // message to help user with next task
    println!(
        r#"
After `cargo auto doc`, check `docs/index.html`. 
run `cargo auto test`,
"#
    );
}

/// cargo test
fn task_test() {
    run_shell_command("cargo test");
   
    println!(
        r#"
After `cargo auto test`, 
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
run `cargo auto publish_to_crates_io_cargo_crev_reviews`
"#
            );
        }
    }
}

/// publish simple_server
fn task_publish_to_crates_io_cargo_crev_reviews() {
    unwrap!(std::env::set_current_dir("cargo_crev_reviews"));
    let cargo_toml = CargoToml::read();
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    run_shell_command(&shell_command);
    // cargo publish
    run_shell_command("cargo publish");
    unwrap!(std::env::set_current_dir(".."));
    println!(
        r#"
After `cargo auto publish_to_crates_io_cargo_crev_reviews', 
check `https://crates.io/crates/{package_name}`.
Install the new version 
`cargo install cargo_crev_reviews`
and try it 
`cargo_crev_reviews`
"#,
        package_name = cargo_toml.package_name()
    );
}
// endregion: tasks
