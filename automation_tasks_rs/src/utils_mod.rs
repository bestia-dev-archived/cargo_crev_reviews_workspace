// utils_mod.rs

use unwrap::unwrap;

/// functions must be prefixed and start with pub fn
pub fn list_methods(file_path:&str, function_prefix:&str )->Vec<String>{
    let mut vec:Vec<String>=vec![];
    let code = unwrap!(std::fs::read_to_string(file_path));
    let mut cursor = 0;
    let functions_starts_with = format!("pub fn {}", function_prefix);
    while let Some(range) = find_range_between_delimiters(&code,&mut cursor, &functions_starts_with, "("){
        vec.push(format!("{}{}",function_prefix, &code[range]));
    }
    vec
}


pub fn replace_delimited_segment(file_path:&str, replace_with: String, start_delimiter:&str, end_delimiter:&str ) {    
    let old_generated = unwrap!(std::fs::read_to_string(file_path));
    let range = unwrap!(find_range_between_delimiters(&old_generated,&mut 0,start_delimiter, end_delimiter));
    let mut new_generated = String::with_capacity(old_generated.len());
    new_generated.push_str(&old_generated[..range.start]);
    new_generated.push_str("\n");
    new_generated.push_str(&replace_with);
    new_generated.push_str(&old_generated[range.end..]);
    unwrap!(std::fs::write(file_path, new_generated));
}

/// find and return the range of the first occurrence between start and end delimiters
/// Success: mutates also the cursor position, so the next find will continue from there
/// Fail: return None if not found and don't mutate pos_cursor
/// I use type Range to avoid references &str and lifetimes. But the programmer can make
/// the error to apply the range to the wrong vector.
pub fn find_range_between_delimiters(source_str: &str, pos_cursor: &mut usize, start_delimiter: &str, end_delimiter: &str) -> Option<std::ops::Range<usize>> {
    if let Some(pos_start) = find_pos_after_delimiter(source_str, *pos_cursor, start_delimiter) {
        // dbg!(&pos_start);
        if let Some(pos_end) = find_pos_before_delimiter(source_str, pos_start, end_delimiter) {
            // dbg!(&pos_end);
            *pos_cursor = pos_end + end_delimiter.len();
            return Some(pos_start..pos_end);
        }
    }
    // return
    None
}


/// return the position after the delimiter or None
/// Does NOT mutate the pos_cursor, because that is for a higher level logic to decide.
pub fn find_pos_after_delimiter(source_str: &str, pos_cursor: usize, delimiter: &str) -> Option<usize> {
    //
    if let Some(pos) = find_from(source_str, pos_cursor, delimiter) {
        let pos = pos + delimiter.len();
        return Some(pos);
    }
    // return
    None
}

/// return the position before the delimiter or None
/// Does NOT mutate the pos_cursor, because that is for a higher level logic to decide.
pub fn find_pos_before_delimiter(source_str: &str, pos_cursor: usize, delimiter: &str) -> Option<usize> {
    if let Some(pos) = find_from(source_str, pos_cursor, delimiter) {
        return Some(pos);
    }
    // return
    None
}

/// find str from pos_cursor low level
/// panics if pos_cursor is incorrect: Check for bugs in calling functions.
pub fn find_from(source_str: &str, pos_cursor: usize, find_str: &str) -> Option<usize> {
    let slice01 = unwrap!(source_str.get(pos_cursor..));
    let option_pos_found = slice01.find(find_str);
    if let Some(pos_found) = option_pos_found {
        // return Option with usize
        Some(pos_cursor + pos_found)
    } else {
        // return
        None
    }
}