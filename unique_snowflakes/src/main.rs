use std::env;
use std::array;
use std::fs::File;
use std::io::{self, BufRead, LineWriter, Write};
use std::path::Path;

fn main() {

    let test_arr: [u32; 5] = [1,2,3,1,5];
    let test_arr_2: [u32; 5] = [1,2,3,1,5];
    println!("{}", is_identical(&test_arr, &test_arr_2));

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

/* Combined the two function so we can make sure they are identical  */
fn is_identical(snow1: &[u32], snow2: &[u32]) -> bool{
    for start in 0..5{
        if identical_right(snow1, snow2, start) {
            return true;
        }
        else if identical_left(snow1, snow2, start){
            return true;
        }
    }
    return false;
}

/* Check if two array are the same when checking from the right*/
fn identical_right(snow1: &[u32], snow2: &[u32], start: u32) -> bool{
    for offset in 0..5 {
        if snow1[offset as usize] != snow2[((start + offset) % 6) as usize]{
            return false;
        }
    }
    return true;
}

/* Check if two array are the same when checkign from the left  */
fn identical_left(snow1: &[u32], snow2: &[u32], start: u32) -> bool{
    for offset in 0..5 {
        if snow1[offset as usize] != snow2[((start - offset) % 6) as usize]{
            return false;
        }
    }
    return true;
}
