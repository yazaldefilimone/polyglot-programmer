struct User {
    email: String,
    username: String,
    sign_inc_count: u64,
    active: bool,
}
struct UserBasic {
    email: String,
    username: String,
}
fn main() {
    let user: User = return_an_struct();

    print!(
        "email:{}, username:{},sign:{}, active: {}",
        user.email, user.username, user.sign_inc_count, user.active
    );

    let user_assign_with_basic: User = User {
        active: true,
        sign_inc_count: 1,
        ..user
    };

    let user_builded: UserBasic = build_user(
        user_assign_with_basic.username,
        user_assign_with_basic.email,
    );
    print!("{}", user_builded.email);
    print!("{}", user_builded.username);
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

fn build_user(username: String, email: String) -> UserBasic {
    let user: UserBasic = UserBasic { username, email };
    user
}
