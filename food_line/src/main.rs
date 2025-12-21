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

    /* The following line read the content*/
    let first_line: Vec<&str> = file_content[0].split_whitespace().collect();
    let people_in_line: i32 = first_line[1].parse().expect("Not a valid number");
    
    /* Create an mutable vector for number of people in the lines*/
    let mut lines: Vec<i32> = file_content[1]
                            .split_whitespace()
                            .map(|s|s.parse().unwrap())
                            .collect();

    /* Create new vector to store the order*/
    let mut order_of_line: Vec<u32> = vec![];

    for _n in 0..people_in_line {
        let _ = &order_of_line.push(update_line(&mut lines));
    }
    
    /* Write the output to an output file*/
    let _ = write_file(String::from("Output.txt"), &order_of_line);

}

/* A function to read a file to Buffer  */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/* A function to write a file  */ 
fn write_file(filename: String, output: &Vec<u32>) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut output_file = LineWriter::new(file);
    for line in output {
        let to_write = line.to_string();
        let write_byte = to_write.as_bytes();
        let _ = output_file.write_all(&write_byte);
        let _ = output_file.write_all(b"\n");
    }
    let _ = output_file.flush()?;
    Ok(())
}

/* Find the shortest line */
fn ret_shortest_line(lines: &Vec<i32>) -> i32 {
    let mut shortest: Option<i32> = None;

    for item in lines{
        match shortest{
            Some(x) => if item < &x{
                shortest = Some(*item)            
            },
            None => {
                shortest = Some(*item)            
            }
        }
    }

    match shortest{
        Some(x) => return x,
        None => return 0
    }
}


/* update the array and return the shortest route */
fn update_line(lines: &mut Vec<i32>) -> u32 {
    let shortest:i32 = ret_shortest_line(lines);

    for line in lines.iter_mut() {
        if *line == shortest {
           *line +=1;
           break;
        }
    }

    return shortest as u32;

}
