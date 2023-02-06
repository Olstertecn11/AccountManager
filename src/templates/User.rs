pub struct User{
    username: String,
    password: String,
    email: String,
    description: String
}


impl User{
    pub fn new(username: String, pass: String, email: String, desc: String)->Self{
        User{
            username,
            password: pass,
            email,
            description: desc
        }
    }

    pub fn _print(&self){
        println!("user: {} -- pass: {} -- email: {} -- desc: {}", self.username, self.password, self.email, self.description);
    }
}
