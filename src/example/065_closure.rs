pub fn run() {
    let weak_password = "password123".to_owned();
    let _validator = |username: &str, password: &str| !username.is_empty() && !password.is_empty();
    let validator2 = |username: &str, password: &str| {
        !username.is_empty()
            && !password.is_empty()
            && password.len() > 8
            && password.contains(['!', '@', '#', '%', '$', '^', '&'])
            && password != weak_password
    };
    let credentials = Credential {
        username: "admin".to_owned(),
        password: "password123".to_owned(),
        validator: validator2,
    };
    let _res = (credentials.validator)(&credentials.username, &credentials.password);

    // println!(
    //     "{}",
    //     validator(&credentials.username, &credentials.password)
    // );

    // println!("{}", res);

    println!("{}", credentials.is_valid());
}

struct Credential<T>
where
    T: Fn(&str, &str) -> bool,
{
    username: String,
    password: String,
    validator: T,
}

impl<T> Credential<T>
where
    T: Fn(&str, &str) -> bool,
{
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn _valid_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}
