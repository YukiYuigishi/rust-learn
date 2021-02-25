#![allow(unused)]
struct User {
    username: String,
    email:String,
    sign_in_count:u64,
    active:bool,
}

fn main() {

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("anoter@example.com"),
    username: String::from("anoterusername567"),
    ..user1
};

}

fn build_user(email: String,username:String) -> User{
    User {
         email,
         username,
        active: true,
        sign_in_count: 1,

    }
}
