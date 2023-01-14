pub fn read(input_string: &str) -> String {
    let mut string = input_string.to_owned();
    string.pop();
    string
}

pub fn eval(input_string: &str) -> String {
    input_string.to_owned()
}

pub fn print(input_string: &str) {
    println!("{}", input_string.to_owned());
}

pub fn rep(input_string: &str) {
    print(&eval(&read(input_string)));
}
