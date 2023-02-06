pub struct User{
    username: String,
    password: String,
    email: String,
    description: String
}


impl User{
    pub fn new(username: String, pass: String, email: String, desc: String)->Self{
        User{
            username: username,
            password: pass,
            email: email,
            description: desc
        }
    }
}
