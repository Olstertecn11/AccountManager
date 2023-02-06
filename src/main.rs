mod templates;
use crate::{templates::User::User, utils::{readline, read_number, clear, pause}};
mod utils;


fn main(){
    // let usr: User = User::new("Oliver".to_string(), "daniela11".to_string(), 
                              // "olstertecn597@gmail.com".to_string(), "outlook".to_string());
    // usr._print();
    let _users: Vec<User> = Vec::new();
    let _run: bool = true;
    while _run {
        clear();
        println!("1- Add User");
        println!("2- List Users");
        println!("3- Exit");
        println!("Insert a option: ");
        let option: i32 = read_number();

        if option == 1{
            clear();
            let name: String = readline("Ingrese un nombre: ".to_string());
            let pass: String = readline("Ingrese una contraseña: ".to_string());
            let email: String = readline("Ingrese un correo: ".to_string());
            let desc: String = readline("Ingrese una descripcion de la cuenta: ".to_string());
            let new_user: User = User::new(name, pass, email, desc);
            _users.push(new_user);
            pause();
        }
        if option == 2{
            clear();
            for usr in _users{
                usr._print();
            }
            pause();
        }
        
        if option == 3{
            clear();
            _run = false;
            pause();
        }
    }

}
