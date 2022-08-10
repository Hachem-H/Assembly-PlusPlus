#[allow(dead_code)]
pub mod x32;
#[allow(dead_code)]
pub mod x64;

#[allow(dead_code)]
pub fn get_file_descriptor(name: &str) -> Option<i32> {
    match name {
        "STDIN" => Some(0),
        "STDOUT" => Some(1),
        "STDERR" => Some(2),
        _ => None
    }
}
