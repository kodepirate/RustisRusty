struct User {
    username: String,
    email: String,
    active_user: bool,
    signin_count: u64
}


fn main() {
    let user1 = User{
        email: String::from("akash!mail.com"),
        username: String::from("Akash"),
        active_user: true,
        signin_count:1
    };

    let name  = user1.username;    
}
