//! utils_mod.rs

/*
/// replace wt (web text) placeholder form the template
/// wt looks like this: `<div><!--wt_unit-->EUR</div>`
/// on the left is delimited with a comment, on the right with `<`
pub fn replace_wt_placeholder(source_str: &str, wt_name: &str, replace_with: &str) -> String {
    let start_delimiter = &format!("<!--{}-->", wt_name);
    let end_delimiter = "<";
    let pos_cursor = 0;
    if let Some(pos_start) = find_pos_before_delimiter(source_str, pos_cursor, start_delimiter) {
        if let Some(pos_end) = find_pos_before_delimiter(source_str, pos_start + start_delimiter.len(), end_delimiter) {
            let mut new_text = source_str[..pos_start].to_owned();
            new_text.push_str(replace_with);
            new_text.push_str(&source_str[pos_end..]);
            return new_text;
        }
    }
    // return
    source_str.to_owned()
}
*/

/*
/// get a random number u32
pub fn get_random_u32() -> u32 {
    (js_sys::Math::random() * u32::MAX as f64) as u32
}
 */
