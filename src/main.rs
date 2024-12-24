use rand::distributions::*;
use rand::prelude::*;

#[derive(Debug)]
enum GenType {
    Number,
    UpperCase,
    LowerCase,
}

impl Distribution<GenType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GenType {
        match rng.gen_range(0..=2) {
            0 => GenType::Number,
            1 => GenType::UpperCase,
            _ => GenType::LowerCase,
        }
    }
}

/// Randomly generates a token.
///
/// A "token" is a `String` of 6 characters:
/// - An ASCII uppercase letter (**A** to **Z**)
/// - An ASCII lowercase letter (**a** to **z**)
/// - An ASCII digit (**0** to **9**)
fn generate_token(rng: &mut ThreadRng) -> String {
    let mut token = String::new();

    for _ in 0..6 {
        let token_item_type: GenType = rng.gen();
        let token_item = match token_item_type {
            GenType::Number => rng.gen_range(0x30..=0x39),
            GenType::UpperCase => rng.gen_range(0x41..=0x5a),
            GenType::LowerCase => rng.gen_range(0x61..=0x7a),
        };
        let token_char = char::from_u32(token_item).expect("failed to parse result into char");

        token.push(token_char);
    }

    token
}

fn generate_password(rng: &mut ThreadRng, pos: usize) -> String {
    let mut tokens = vec!["".to_string(); pos];
    tokens.iter_mut().for_each(|t| *t = generate_token(rng));
    let last_token = tokens.pop().expect("failed to generate any tokens");

    let mut password = String::new();
    for token in tokens {
        password.push_str(&token);
        password.push('-');
    }
    password.push_str(&last_token);

    password
}

fn main() {
    let mut rng = thread_rng();

    // Create a password of 3 tokens, separated by dashes
    // Schema comes from what Apple's "Password" macOS app generates.
    // let tokens: [String; 3] = core::array::from_fn(|_| generate_token(&mut rng));
    // let Some((last, rest)) = tokens.split_last() else {
    //     panic!("password failed to generate");
    // };
    // let mut password = String::new();
    // for token in rest {
    //     password.push_str(token);
    //     password.push('-');
    // }
    // password.push_str(last);

    println!("{}", generate_password(&mut rng, 3));
}
