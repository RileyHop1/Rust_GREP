use std::{
  env,
  fs::File,
  io::Read
};


fn main() {


    //grabs the input arguments8
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Please provide a file path and keyword");
        return;
    }

    let file_path = &args[1];
    let mut key_word  = args[2].clone();

    let flag: &str;

    if let Some(arg_flag) = args.get(3) {
        flag = arg_flag;
    } else {
        flag = "-d";
    }


    let mut file = File::open(file_path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Cannot read file");

    let mut key_word_lines: Vec<&str> = Vec::new();


    match flag {
        "-i"=> {
            contents = contents.to_lowercase();
            key_word = key_word.to_lowercase();
        },
        "-d"=> {},
        _=>  panic!("Invalid flag state"),
    }

    //State machine that tracks if word is found in line.
    let key_word_vec: Vec<char> = key_word.chars().collect();
    let mut pos: u32 = 0;

    for line in contents.lines() {
        for word in line.split_whitespace().collect::<Vec<&str>>().iter() {
            let mut word_length = word.len();
            let key_length = key_word.len();

            if word_length < key_length   {
                continue;
            }


            for c in word.chars() {
              if word_length < key_length - pos as usize {
                  break;
              } else if c == key_word_vec[pos as usize] {
                  pos = pos + 1;

              } else {
                  pos = 0;
              }
                if pos == key_word_vec.len() as u32 {
                    key_word_lines.push(line);
                    pos = 0;
                }

                word_length -= 1;
            }
        }


    }
    for line in key_word_lines.iter() {
        println!("{}", line);
    }

}
