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

    let password_validator = get_password_validator(8, true);

    // let default_cred = get_default_cred(validator2);
    let default_cred = get_default_cred(password_validator);
    println!(
        "Username: {}, Password: {}",
        default_cred.username, default_cred.password
    );
}

#[derive(Debug)]
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

fn get_default_cred<T>(f: T) -> Credential<T>
where
    T: Fn(&str, &str) -> bool,
{
    Credential {
        username: "guest".to_owned(),
        password: "password123".to_owned(),
        validator: f,
    }
}

fn get_password_validator(min_len: usize, special_char: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    if special_char {
        Box::new(move |_: &str, password: &str| {
            !password.len() > min_len && password.contains(['!', '@', '#', '%', '$', '^', '&'])
        })
    } else {
        Box::new(move |_: &str, password: &str| !password.len() > min_len)
    }
}

// return one
fn _get_password_validator(min_len: usize) -> impl Fn(&str, &str) -> bool {
    move |_: &str, password: &str| !password.len() > min_len
}
