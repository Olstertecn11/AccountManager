use std::io;
use std::io::prelude::*;

pub fn readline(prompt: String)-> String{
    println!("{}", prompt);
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    b1.to_string()
}

pub fn clear()-> (){
    std::process::Command::new("clear").status().unwrap();
}

pub fn pause(){
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn read_number()-> i32{
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read from stdin");


    let trimmed = number.trim();
    let aint: i32 = trimmed.trim().parse().ok().expect("Error in user input");
    match trimmed.parse ::<i32>(){
        Ok(_i)=> println!(""),
        Err(..)=> println!("This was not an integer: {}", number),
    };
    return aint;
}



