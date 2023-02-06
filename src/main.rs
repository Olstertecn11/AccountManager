mod templates;
use crate::templates::User::User;


fn main(){
    let usr: User = User{
        username: String::from("Oliver"),
        password: String::from("daniela11"),
        email: String::from("olstertecn597@gmail.com"),
        description: String::from("jfksdkflsajflk")
    };

}
