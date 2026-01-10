use std::env;
use std::array;
use std::fs::File;
use std::io::{self, BufRead, LineWriter, Write};
use std::path::Path;

fn main() {

    let test_arr: [u32; 5] = [1,2,3,1,5];

    identical_right(&test_arr, &test_arr, 0);

    // let args: Vec<String> = env::args().collect();

    // let file_path = &args[1];

    // // Create a vector to store the file_content
    // let mut file_content = Vec::new();

    // /* This following line read the input using the buffer function  and store them into a vector*/
    // if let Ok(lines) = read_lines(file_path) {
    //     for line in lines.map_while(Result::ok){
    //         let _ = &file_content.push(line);
    //     }
    // }


}

/* A function to read a file to Buffer  */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/* Check if two integers are the same */
fn identical_right(snow1: &[u32], snow2: &[u32], start: u32) {
    let (offset, snow2_index): (u32,u32);
    return
}

