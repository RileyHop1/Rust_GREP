use std::{
  env,
  fs::File,
  io::Read
};


/*
For each key in a line this will hold the line it is within,
The file it's from, and the line number that it exists within.
*/

#[derive()]
struct KeyLine {
    line: String,
    file_name: String,
    line_num: u32,
}

impl KeyLine {
    pub fn new(line: String, file_name: String, line_num: u32) -> Self {
        KeyLine {
            line,
            file_name,
            line_num,
        }
    }

    pub fn print_line(&self) {
        println!("{}:\n{}", self.file_name, self.line);
    }

    pub fn print_line_and_line_number(&self) {

        println!("{}:\n{}:{}", self.file_name, self.line_num, self.line);
    }
}

fn find_key_word_lines(path: &str ,contents: &str, key: &str) -> Vec<KeyLine> {

    let mut key_word_lines: Vec<KeyLine> = Vec::new();
    let mut line_number: u32 = 0;

   for line in contents.lines() {

       if line.contains(key) {

           key_word_lines.push(
               KeyLine::new(
                   line.to_string(),
                   path.to_string(),
                   line_number
               )
           );

       }
       line_number += 1;

   }
    key_word_lines
}

fn main() {


    //grabs the input arguments8
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Please provide a file path and keyword");
        return;
    }

    let file_path = args[1].clone();
    let key_word  = args[2].clone();

    let flag: &str;

    if let Some(arg_flag) = args.get(3) {
        flag = arg_flag;
    } else {
        flag = "-d";
    }

    let mut file = File::open(&file_path).expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Cannot read file");

    let key_word_lines = find_key_word_lines(&file_path, &contents, &key_word);

    for key_line in key_word_lines {
        key_line.print_line_and_line_number();
    }


}
