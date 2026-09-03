use std::{fs::File, io::ErrorKind};
use std::io::{self,Read};

fn main(){
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("ERROR BITCH {e:?}"),
            },
            _ => {
                panic!("couldnt open file")
            }
        }
    };

    let hello_file = File::open("hello.txt")
        .expect("Eror opening the file");

    let _ = read_username_from_file();

}

fn read_username_from_file() -> Result<String, io::Error>{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
