// routing_local_hash_mod.rs

use crate::cln_methods_mod;
use crate::web_sys_mod as w;

/// read the url hash parameters for local routing
/// this is a SPA single page application. The page is always index.html
/// then a hash parameter is added for local routing like index.html#edit/crate_name/crate_version
/// the main page is opened only once. It lists the cargo_tree and verify all the dependencies.
/// All other pages are opened in separate tabs. So the user can easily close this tabs and return to the main page.
/// The use of the back button in not recommended.
fn router_for_local_hash_routing(param1: &str, param2: &str, param3: &str) {
    // param1 is the "routing method" name
    match param1 {
        "edit_or_new" => cln_methods_mod::cln_review_list_mod::routing_edit_or_new(param2, param3),
        "version_list" => cln_methods_mod::cln_version_mod::routing_version_list(param2),
        "publisher_list" => cln_methods_mod::cln_publisher_list_mod::routing_publisher_list(),
        "config_edit" => cln_methods_mod::cln_config_mod::routing_config_edit(),
        _ => log::info!("unrecognized hash routing method: {}", param1),
    }
}

/// get 3 param from hash
/// example "#edit/crate_name/crate_version" -> ["edit","crate_name","crate_version"]
/// if the param does not exist returns an empty string
pub fn get_3_url_param_from_hash(location_hash: &str) -> (&str, &str, &str) {
    let mut spl = location_hash.trim_start_matches("#").split('/');
    (spl.next().unwrap_or(""), spl.next().unwrap_or(""), spl.next().unwrap_or(""))
}

/// jump over this boilerplate to router_for_local_hash_routing()
pub fn router_boilerplate() {
    fn open_main_page() {
        cln_methods_mod::cln_cargo_tree_mod::request_cargo_tree_list("");
    }
    let location: web_sys::Location = w::window().location();
    match location.hash() {
        // if there is no url hash, show the first page: cargo_tree
        Err(_err) => open_main_page(),
        Ok(location_hash) => {
            if location_hash.is_empty() {
                open_main_page();
            } else {
                let (param1, param2, param3) = get_3_url_param_from_hash(&location_hash);
                router_for_local_hash_routing(param1, param2, param3);
            }
        }
    }
}
