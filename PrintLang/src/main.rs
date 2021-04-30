use std::fs::File;
use std::io::{BufRead, BufReader};
use std::borrow::Borrow;

fn main() {
    let filename = "src/test.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);


    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors
        if(line.contains("PRINT_THE_LINE")){
            for x in 1..line.len(){
                if(line.get(x - 1..x) == Some("(")) {
                    let start = x;
                    for n in x..line.len() {
                        if (line.get(n - 1..n) == Some(")")) {
                            let test = line.get(x..n - 1);
                            printer(test);
                            break;
                        }
                    }
                    break;
                }
            }
        }
        // Show the line and its number.

    }


}

fn printer(print_string:Option<&str>){
    println!("{:?}", print_string);
}

