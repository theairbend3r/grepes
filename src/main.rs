use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    // read command line arguments
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for '{query}' in {file_path}.\n");

    // open file and load contents
    let f = fs::File::open(file_path).expect("file not found");
    let reader = io::BufReader::new(f);

    // search for term inside file
    for line in reader.lines() {
        let line = line.expect("Could not read file");
        if line.contains(query) {
            println!("{line}");
        }
    }
}
