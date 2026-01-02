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


/// Finds all lines in a file that contain the given keyword (case-sensitive).
///
/// This function scans the provided file contents line-by-line and
/// returns a list of [`KeyLine`] entries for each matching line.  
/// Matching is **case-sensitive**.
///
/// # Arguments
///
/// * `path` — Path to the file being searched. Used only for metadata.
/// * `contents` — Full text contents of the file.
/// * `key` — Keyword to search for (case-sensitive).
///
/// # Returns
///
/// A vector of [`KeyLine`] entries, one for every line that contains `key`.
fn find_key_word_lines_d(path: &str ,contents: &str, key: &str) -> Vec<KeyLine> {

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

/// Finds all lines in a file that contain the given keyword (case-insensitive).
///
/// This function converts both the `contents` and `key` to lowercase internally
/// before performing the search. It returns a list of [`KeyLine`] entries for
/// each matching line.
///
/// # Arguments
///
/// * `path` — Path to the file being searched. Used only for metadata.
/// * `contents` — Full text contents of the file.
/// * `key` — Keyword to search for (case-insensitive).
///
/// # Returns
///
/// A vector of [`KeyLine`] entries, one for every line that matches `key`,
/// ignoring case.
fn find_key_word_lines_i(path: &str ,contents: &str, key: &str) -> Vec<KeyLine> {

    let mut key_word_lines: Vec<KeyLine> = Vec::new();
    let mut line_number: u32 = 0;

    let key = key.to_lowercase();

    for line in contents.lines() {
        let line_i = line.to_lowercase();
        if line_i.contains(&key) {

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

/// Prints the program usage guide and available command-line flags.
///
/// This function displays information about how to correctly use the
/// program, including required arguments and optional flags. It is
/// typically invoked when the user supplies the `-h` or `--help` flag,
/// or when the user provides insufficient or malformed arguments.
fn print_help() {
    println!(
                "Usage: myprog <file> <keyword> [flag]

                Flags:
                -d     Display lines containing the keyword
                -l     Display lines containing the keyword with line numbers
                -i     Case-insensitive search
                -il    Case-insensitive search with line numbers
                -h     Show this help message

                If no flag is provided, the default is -d."
            )
}
fn main() {


    //grabs the input arguments8
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 
        && args[1] == "-h" 
        || args[1] == "--help" {

        print_help();
        return;
    } else if args.len() < 3 {
        println!("Please provid a file path and keyward or -h for commands");
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

    match flag {
        "-h" | "--help" => {
            print_help(); 
        },
        "-d" => {
            let key_word_lines = find_key_word_lines_d(&file_path, &contents, &key_word);

            for key_line in key_word_lines {
                key_line.print_line();
            }

        },
        "-l" => {
            let key_lines = find_key_word_lines_d(&file_path, &contents, &key_word);
            for key_line in key_lines {
                key_line.print_line_and_line_number();
            }
        },
        "-i" => {
            let key_lines = find_key_word_lines_i(&file_path, &contents, &key_word);
            for key_line in key_lines {
                key_line.print_line();
            }
        },
        "-il" => {
            let key_lines = find_key_word_lines_i(&file_path, &contents, &key_word);
            for key_line in key_lines {
                key_line.print_line_and_line_number();
            }
        }
        _=> {
            panic!("Malformed flag");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_key_word_test() {

        let file_path = "src/test.txt";
        let key_word  = "john";

        let mut file = File::open(&file_path).expect("file not found");
        let mut contents = String::new();

        file.read_to_string(&mut contents).expect("Cannot read file");

        let key_word_lines = find_key_word_lines_d(&file_path, &contents, &key_word);

        assert_eq!(key_word_lines[0].line,"Hello my name is john" );
        assert_eq!(key_word_lines[1].line,"john is the name, john I am" );



    }

    #[test]
    fn find_key_word_ignore_case_test() {

        let file_path = "src/test.txt";
        let key_word  = "john";

        let mut file = File::open(&file_path).expect("file not found");
        let mut contents = String::new();

        file.read_to_string(&mut contents).expect("Cannot read file");

        let key_word_lines = find_key_word_lines_i(&file_path, &contents, &key_word);

        assert_eq!(key_word_lines[0].line,"Hello my name is john" );
        assert_eq!(key_word_lines[1].line,"Bob is not my name, its John" );
        assert_eq!(key_word_lines[2].line,"john is the name, john I am" );



    }


}
