use passgen::passwords::*;

fn main() {
    let pass = PasswordSpec {
        token_size: 6,
        token_count: 3,
    };
    println!("{}", generate_password(pass));
}
