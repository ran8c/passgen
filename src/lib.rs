pub mod tokens {
    use rand::distributions::Standard;
    use rand::prelude::*;

    #[derive(Debug)]
    enum SupportedChar {
        Number,
        Upper,
        Lower,
    }

    impl Distribution<SupportedChar> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SupportedChar {
            match rng.gen_range(0..=2) {
                0 => SupportedChar::Number,
                1 => SupportedChar::Upper,
                _ => SupportedChar::Lower,
            }
        }
    }

    pub fn generate_token(rng: &mut ThreadRng, size: usize) -> String {
        let mut token = String::with_capacity(size);

        for _ in 0..size {
            let token_rng: SupportedChar = rng.gen();
            let token_byte: u32 = match token_rng {
                SupportedChar::Number => rng.gen_range(0x30..=0x39),
                SupportedChar::Upper => rng.gen_range(0x41..=0x5a),
                SupportedChar::Lower => rng.gen_range(0x61..=0x7a),
            };
            let token_char = char::from_u32(token_byte).expect("failed to parse result into char");
            token.push(token_char);
        }

        token
    }
}

pub mod passwords {
    use rand::prelude::*;

    use crate::tokens::generate_token;

    pub struct PasswordSpec {
        pub token_size: usize,
        pub token_count: usize,
    }

    pub fn generate_password(pass: PasswordSpec) -> String {
        let mut rng = thread_rng();

        let mut tokens = vec![String::with_capacity(pass.token_size); pass.token_count];
        tokens.iter_mut().for_each(|t| {
            *t = generate_token(&mut rng, pass.token_size);
        });
        let last_token = tokens.pop().expect("failed to generate any tokens");

        let mut password = String::with_capacity(pass.token_count * pass.token_size);
        for token in tokens {
            password.push_str(&token);
            password.push('-');
        }
        password.push_str(&last_token);

        password
    }
}
