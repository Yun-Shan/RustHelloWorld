use my_macro::post;

#[post(a = "123", b = 666)]
pub fn invoke2(
    #[query_arg(name = "1")]
    #[check(max_len = 64, min_len = 32)]
    a: &str,
    #[json_arg(name = "1")]
    #[many_props(a = "hi, master!", b = 2, c = true, d = 6.66, e = ';')]
    b: &str
) {
    println!("hi {}, {}", a, b);
}

#[post(path = "/create")]
pub fn create(
    #[query_arg(name = "username")]
    #[check(max_len = 64, min_len = 32)]
    username: &str,
    #[check(min_len = 16, regex = "^[0-9A-Za-z]+$")]
    #[body_arg(name = "password", contentType = "json")]
    password: &str,
) {
    println!("hi {}, {}", username, password);
}

