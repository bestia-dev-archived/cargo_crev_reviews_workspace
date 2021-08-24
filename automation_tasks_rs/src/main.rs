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
                } else if &task == "docs" || &task == "doc" || &task == "d" {
                    task_docs();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "github_new_release" {
                    task_github_new_release();
                } else if &task == "run" {
                    task_run();
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
cargo auto release - builds the crate in release mode, version from date, fmt
cargo auto increment_minor - increments the semver version minor part (only for libraries)
cargo auto docs - builds the docs, copy to docs directory
cargo auto commit_and_push - commits with message and push with mandatory message
    if you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git
cargo auto github_new_release - creates new release on github
    this task needs PAT (personal access token from github) in the env variable: `export GITHUB_TOKEN=paste_token_here`

"#
    );
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc", "commit_and_push", "github_new_release"];
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

/// build every member of workspace
fn task_build() {
    copy_web_folder_files_into_module();
    #[rustfmt::skip]
    let shell_commands = [
        "cargo fmt", 
        "cargo build"];
    run_shell_commands(shell_commands.to_vec());
    println!(
        r#"
After `cargo auto build`, run the tests and the code. If ok, then 
run`cargo auto release`
"#
    );
}

/// this workspace is basically one single application splitted into 3 projects
/// it deserves the same version number
fn task_release() {
    copy_web_folder_files_into_module();
    //auto_version_from_date();
    //auto_cargo_toml_to_md();
    auto_lines_of_code("");

    run_shell_command("cargo fmt");
    run_shell_command("cargo build --release");
    // run_shell_command(&format!("strip target/release/{}",package_name()));

    println!(
        r#"
After `cargo auto release`, run the code 
with `cargo auto run`. If ok, then 
run `cargo auto doc`
"#
    );
}

/// after release, run the web server and open the browser
fn task_run() {
    run_shell_command("web_server_folder/cargo_crev_reviews_micro_web_server_backend");
    run_shell_command("www http://127.0.0.1:8080/cargo_crev_reviews");
    println!(
        r#"
After `cargo auto run` close the CLI with ctrl+c and close the browser.
"#
    );
}

/// example how to call a list of shell commands and combine with rust code
fn task_docs() {
    auto_md_to_doc_comments();
    #[rustfmt::skip]
    let shell_commands = [
        "cargo doc --no-deps --document-private-items --open",
        // copy target/doc into docs/ because it is github standard
        "rsync -a --info=progress2 --delete-after target/doc/ docs/",
        "echo Create simple index.html file in docs directory",
        &format!("echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",package_name().replace("-","_")) ,
    ];
    run_shell_commands(shell_commands.to_vec());
    // message to help user with next move
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
run `cargo auto github_new_release`
"#
            );
        }
    }
}

/// create a new release on github with octocrab
/// the env variable GITHUB_TOKEN must be set `export GITHUB_TOKEN=paste_token_here`
fn task_github_new_release() {
    // async block inside sync code with tokio
    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let owner = github_owner();
        let repo = package_name();
        let version = package_version();
        let name = format!("Version {}", &package_version());
        let branch = "main";
        let body_md_text = &format!(
            r#"
This is {package_name} from {owner}.
Write something meaningful here. Maybe instructions how to install.
Or maybe what changed in this release.
            "#,
            package_name = package_name(),
            owner = owner
        );

        let release_id = github_create_new_release(&owner, &repo, &version, &name, branch, body_md_text).await;
        println!(
            "New release created, now uploading release asset. This can take some time if the files are big. Wait ..."
        );

        // upload asset
        let path_to_file = format!("target/release/{package_name}", package_name = package_name());

        github_upload_asset_to_release(&owner, &repo, &release_id, &path_to_file).await;
        println!(
            r#"
After `cargo auto github_new_release', 
check `https://github.com/{github_owner}/{package_name}/releases/tag/v{package_version}`.
Download it and check hot it works.  
"#,
            package_name = package_name(),
            package_version = package_version(),
            github_owner = github_owner(),
        );
    });
}

// endregion: tasks

/// copy all files in the web_server_folder as strings to the module `files_mod.rs`
fn copy_web_folder_files_into_module() {
    let mut module_source_code = String::new();
    module_source_code.push_str("// files_mod.rs\n\n");
    copy_files_from_dir("web_server_folder/cargo_crev_reviews", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/css", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/icons", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/images", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/pkg", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/templates", &mut module_source_code);
    unwrap!(std::fs::write(
        "cargo_crev_reviews_micro_web_server_backend/src/files_mod.rs",
        module_source_code
    ));
}

/// read all files and push rust code into module
fn copy_files_from_dir(root_directory: &str, module_source_code: &mut String) {
    let path = std::path::Path::new(root_directory);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let p: std::path::PathBuf = entry.path();
            if p.is_file() {
                let ps = p.to_string_lossy();
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
                let body = if ps.ends_with(".png") || ps.ends_with(".woff2") {
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
