use diff_text::{myers_diff, EditAction};
use std::fs;

pub fn main() {
    let file1 = fs::read_to_string("src/samples/dog.txt").expect("missing file1");
    let file2 = fs::read_to_string("src/samples/cat.txt").expect("missing file2");

    let re: Vec<EditAction> = myers_diff(file1, file2).expect("failed to diff");
    
    println!("result diff: {:?}", re);
}