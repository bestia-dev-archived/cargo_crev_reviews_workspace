// cargo_crev_reviews_special_mod.rs

use unwrap::unwrap;
use crate::utils_mod::*;

/// copy all files in the web_server_folder as strings to the module `auto_generated_files_mod.rs`
pub fn copy_web_folder_files_into_module() {
    /// read all files and push Rust code into module
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
                            "\n\"{}\" => {{\nr##\"",
                            ps.trim_start_matches("web_server_folder")
                        );
                        module_source_code.push_str(&start);

                        // binary files are encoded base64
                        let body = if ps.ends_with(".png") || ps.ends_with(".ico") || ps.ends_with(".woff2") || ps.ends_with(".wasm") {
                            let e = base64::encode(unwrap!(std::fs::read(p)));
                            // it is much easier to have lines of 76 characters in Rust source code.
                            // before decoding base64 I will eliminate \n
                            // Rust string is utf8, but base64 is strictly ascii.
                            // So I have the guarantee 100% that 1 byte = 1 char
                            let multi_line = e.as_bytes().chunks(76).collect::<Vec<_>>().join(&b'\n');
                            unwrap!(String::from_utf8(multi_line))
                        } else {
                            unwrap!(std::fs::read_to_string(p))
                        };
                        module_source_code.push_str(&body);

                        let end = format!("\"##\n}}\n");
                        module_source_code.push_str(&end);
                    }
                }
            }
        }
    }

    let mut module_source_code = String::new();
    module_source_code.push_str("// auto_generated_files_mod.rs\n\n");
    module_source_code.push_str("//! embedded files as Rust code\n\n");
    module_source_code.push_str("// exclude from auto_lines_of_code\n");
    module_source_code.push_str(r#"
/* spell-checker: disable */

use std::borrow::Cow;

// for debug_time it is useful to read from file, so editing files can be made without restarting the server
// but for runtime, the files must be embedded in the code, so only one file is published
// this is done by an automation task in `cargo auto` 

// region: debug mode
/// returns Cow, because in debug I read from files and return a String. 
/// In release I have embedded strings n the code
#[cfg(debug_assertions)]
pub fn get_file_text<'a>(file_name:&str) -> Cow<'a, str>{
    let file_name = format!("web_server_folder{}",file_name);
    let str = std::fs::read_to_string(&file_name).unwrap();
    return Cow::Owned(str);
}

/// always allocated? Maybe I could do better. I don't know.
#[cfg(debug_assertions)]
pub fn get_file_bytes<'a>(file_name:&str) -> Vec<u8>{    
        let file_name = format!("web_server_folder{}",file_name);
        let file_bytes = std::fs::read(&file_name).unwrap();
        return file_bytes;
}

// endregion: debug mode

// region: release mode
/// always allocated? Maybe I could do better. I don't know.
#[cfg(not(debug_assertions))]
pub fn get_file_bytes<'a>(file_name:&str) -> Vec<u8>{
    let str = get_file_text(file_name);
    
    if file_name.ends_with(".png") || file_name.ends_with(".woff2") || file_name.ends_with(".wasm") {
        // I artificially added \n to base64 to make it more text editor friendly
        let file_bytes = str.replace("\n", "");    
        let file_bytes = base64::decode(file_bytes).unwrap();
        return file_bytes;

    } else {
        // file_name.ends_with(".html") || file_name.ends_with(".css") || file_name.ends_with(".js") || file_name.ends_with(".json") {
        let file_bytes =str.as_bytes().to_vec();
        return file_bytes;
    };
}

/// returns Cow, because in debug I read from files and return a String. 
/// In release I have embedded strings n the code
#[cfg(not(debug_assertions))]
pub fn get_file_text<'a>(file_name:&str) -> Cow<'a, str>{
    let str = match file_name{
"#);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/css", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/icons", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/images", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/js", &mut module_source_code);
    copy_files_from_dir("web_server_folder/cargo_crev_reviews/pkg", &mut module_source_code);

    module_source_code.push_str(r#"
        _ => {
            log::error!("File not exists: {}", file_name);
            ""
        }
    };
    return Cow::Borrowed(str);
}
// endregion: release mode
    "#);
    unwrap!(std::fs::write("cargo_crev_reviews/src/auto_generated_files_mod.rs", module_source_code));
}

pub fn common_structs_copy(){
    // copy from cargo_crev_reviews/src/common_structs_mod.rs 
    // to cargo_crev_reviews_wasm/src/auto_generated_mod inside common_structs module 
    let code = unwrap!(std::fs::read_to_string("cargo_crev_reviews/src/common_structs_mod.rs"));
    let old_generated = unwrap!(std::fs::read_to_string("cargo_crev_reviews_wasm/src/auto_generated_mod.rs"));
    let range = unwrap!(find_range_between_delimiters(&old_generated,&mut 0,"// region: generated common_structs_mod", "// endregion: generated common_structs_mod"));
    let mut new_generated = String::with_capacity(old_generated.len());
    new_generated.push_str(&old_generated[..range.start]);
    new_generated.push_str("\n");
    new_generated.push_str(&code);
    new_generated.push_str(&old_generated[range.end..]);
    unwrap!(std::fs::write("cargo_crev_reviews_wasm/src/auto_generated_mod.rs", new_generated));
}

pub fn generate_server_methods(){    
    // list functions starting with `pub fn srv_` in files starting with `srv_methods_`
    let mut function_list = list_methods_in_files("cargo_crev_reviews/src/","srv_methods_","srv_");
    function_list.sort();

    let mut code = String::new();
    for function_name in function_list{
        let temp = format!(r#"
#[named]
pub fn {}<T>(request_data: T)
where
    T: serde::Serialize,
{{
    let request_method = function_name!();
    post_request_await_run_response_method(request_method, request_data);
}}
"#, &function_name);
        code.push_str(&temp);
    }
    
    replace_delimited_segment("cargo_crev_reviews_wasm/src/auto_generated_mod.rs",code,"// region: generated srv_methods", "// endregion: generated srv_methods");   
}

pub fn generate_client_match_response_method(){
    
    let mut function_list = list_methods_in_files("cargo_crev_reviews_wasm/src/","cln_methods_","cln_");
    function_list.sort();

    let mut code = String::new();
    for function_name in function_list{
        let temp = format!(r#"
        "{}" => {}(response),"#, &function_name, &function_name);
        code.push_str(&temp);
    }
    code.push_str("\n");
    replace_delimited_segment("cargo_crev_reviews_wasm/src/auto_generated_mod.rs",code,"// region: generated match_response_method", "// endregion: generated match_response_method");   
}

pub fn generate_client_methods(){    
    let mut function_list = list_methods_in_files("cargo_crev_reviews_wasm/src/","cln_methods_","cln_");
    function_list.sort();
    
    let mut code = String::new();
    for function_name in function_list{
        let temp = format!(r#"
#[named]
pub fn {}<T>(response_data: T, response_html: &str) -> anyhow::Result<String>
where
    T: serde::Serialize,
{{
    let response_method = function_name!();
    Ok(return_srv_response(response_method, response_data, response_html))
}}
"#, &function_name);
        code.push_str(&temp);
    }
    
    replace_delimited_segment("cargo_crev_reviews/src/auto_generated_mod.rs",code,"// region: generated cln_methods", "// endregion: generated cln_methods");   
}

pub fn generate_server_match_response_method(){
    let mut function_list = list_methods_in_files("cargo_crev_reviews/src/","srv_methods_","srv_");
    function_list.sort();

    let mut code = String::new();
    for function_name in function_list{
        let temp = format!(r#"
        "{}" => {}(request_data),"#, &function_name, &function_name);
        code.push_str(&temp);
    }
    code.push_str("\n");
    replace_delimited_segment("cargo_crev_reviews/src/auto_generated_mod.rs",code,"// region: generated match_response_method", "// endregion: generated match_response_method");   
}
