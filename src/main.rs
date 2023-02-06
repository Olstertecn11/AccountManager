mod templates;
use crate::templates::User::User;


fn main(){
    let usr: User = User::new("Oliver".to_string(), "daniela11".to_string(), 
                              "olstertecn597@gmail.com".to_string(), "outlook".to_string());
    
    usr._print();

}
