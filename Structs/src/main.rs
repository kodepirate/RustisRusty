struct User {
    username: String,
    email: String,
    active_user: bool,
    signin_count: u64
}


fn main() {
    let mut user1 = User{
        email: String::from("akash!mail.com"),
        username: String::from("Akash"),
        active_user: true,
        signin_count:1
    };

    let name  = user1.username;    // dot notaion // We can modify our structs using dot notation but we need to make the user to mutable
    user1.username = String:: from("@gmail.com");

    let user2 = build_user(String::from("fuciik@gmail.com"), String::from("kodeportal"));
}

fn build_user(email: String, username:String)-> User{
    User{
        email, // This is called the field init shorthand
        username,
        active_user: true,
        signin_count:1,
    }
}