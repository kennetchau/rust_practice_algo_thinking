use std::env;
use std::fs::File;
use std::io::{self, BufRead, LineWriter, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    // Create a vector to store the file_content
    let mut file_content = Vec::new();

    /* This following line read the input using the buffer function  and store them into a vector*/
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok){
            let _ = &file_content.push(line);
        }
    }
    let first_line: Vec<&str> = file_content[0].split_whitespace().collect();

    let people_in_line: i32 = first_line[1].parse().expect("Not a valid number");

}

/* A function to read a file to Buffer  */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/* */

