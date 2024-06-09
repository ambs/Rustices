use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct  Notebook(Vec<Vec<String>>);

fn main() {
    let filename = "samplenote".to_owned();

    let mut contents = Notebook::from_file(filename);
    contents.list();

    contents.add("note4\nnote4".to_owned());

    contents.list();
}

impl Notebook {
    fn add(&mut self, string: String) {
        let lines = string.split("\n").map(|x| x.to_string()).collect();
        self.0.push(lines);
    }

    fn list(&self) {
        let mut cnt = 0;
        for block in &self.0 {
            cnt = cnt + 1;
            for line in block {
                println!("{}", line)
            }
            if cnt < self.0.len() {
                println!("------------------------------")
            }
        }
    }

    fn from_file(path: String) -> Notebook {
        // Open the file
        let file = File::open(path);
        // Did we succeed?
        let buf_reader = match file {
            Ok(fh) => BufReader::new(fh),
            Err(err) => panic!("Error: {err}")
        };

        // Notes contents
        let mut contents = vec![];
        // Store each block
        let mut block = vec![];

        // Read the lines from the file
        for line in buf_reader.lines() {
            // get the contents of `Result<string>`
            let cline = line.unwrap();
            // is this the delimiter?
            if cline.is_empty() {
                contents.push(block);
                block = vec![];
            } else {
                block.push(cline);
            }
        }

        if !block.is_empty() {
            contents.push(block);
        }

        Notebook(contents)
    }
}