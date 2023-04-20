struct User {
    email: String,
    username: String,
    sign_inc_count: u64,
    active: bool,
}
fn main() {
    let user: User = return_an_struct();

    print!(
        "email:{}, username:{},sign:{}, active: {}",
        user.email, user.username, user.sign_inc_count, user.active
    );
}

fn return_an_struct() -> User {
    let user: User = User {
        email: String::from("jhoedue@exe.com"),
        username: String::from("joedue"),
        sign_inc_count: 2,
        active: false,
    };

    user
}
