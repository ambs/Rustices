use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

#[derive(Debug)]
struct  Notebook {
    filename: String,
    notes: Vec<Vec<String>>
}

fn main() {
    let notebook =  Notebook::from_file("samplenote".to_owned());

    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.len() {
        1 => show_usage(),
        _ => handle_argument(&notebook,&args[1], &args[2..]),
    }
}

fn handle_argument(notebook: &Notebook, command: &String, args: &[String]) {
    match command.as_str() {
        "list"   if args.len() == 0 => notebook.list(),
        "search" if args.len() == 1 => notebook.list_search(&args[0]),
        _ => show_usage()
    }
}


fn show_usage() {
    todo!()
}

impl Notebook {

    fn search(&self, query: &str) -> Vec<&Vec<String>> {
        self.notes.iter().filter(|&n| Self::search_in_note(n, query)).collect()
    }

    fn search_in_note(note: &Vec<String>, query: &str) -> bool {
        note.iter().any(|line| line.contains(query))
    }

    fn save(&self) {
        let fh = match File::create(&self.filename) {
            Ok(fh) => fh,
            Err(err) => panic!("Error: {err}!")
        };

        let mut writer = BufWriter::new(fh);
        let mut cnt = 0;
        for block in &self.notes {
            cnt = cnt + 1;
            for line in block {
                write!(writer, "{}\n", line).expect("Error adding text to file");
            }
            if cnt < self.notes.len() {
                write!(writer, "\n").expect("Error adding text to file");
            }
        }
    }

    fn add(&mut self, string: String) {
        let lines = string.split("\n").map(|x| x.to_string()).collect();
        self.notes.push(lines);
    }

    fn list(&self) {
        let mut cnt = 0;
        for block in &self.notes {
            cnt = cnt + 1;
            for line in block {
                println!("{}", line)
            }
            if cnt < self.notes.len() {
                println!("------------------------------")
            }
        }
    }

    fn list_search(&self, query: &str) {
        let hits = self.search(query);

        if hits.len() == 0 {
            println!("No results match '{query}'");
        }
        else {
            let mut cnt = 0;
            for &block in &hits {
                cnt = cnt + 1;
                for line in block {
                    println!("{}", line)
                }
                if cnt < hits.len() {
                    println!("------------------------------")
                }
            }
        }
    }

    fn from_file(path: String) -> Notebook {
        // Open the file
        let file = File::open(&path);
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

        Notebook{filename: path, notes: contents}
    }
}