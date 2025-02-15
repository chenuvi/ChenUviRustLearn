pub fn run() {
    let validator = |username: &str, password: &str| !username.is_empty() && !password.is_empty();
    let credentials = Credential {
        username: "admin".to_owned(),
        password: "123".to_owned(),
        validator,
    };
    let _res = (credentials.validator)(&credentials.username, &credentials.password);

    // println!(
    //     "{}",
    //     validator(&credentials.username, &credentials.password)
    // );

    // println!("{}", res);
}

struct Credential<T>
where
    T: Fn(&str, &str) -> bool,
{
    username: String,
    password: String,
    validator: T,
}

fn _valid_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}
