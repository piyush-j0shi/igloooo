use rand::Rng;

fn main() {
    let password_length = 12;
    let include_uppercase = true;
    let include_numbers = true;
    let include_symbols = true;
    let number_of_passwords = 5;

    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "!@#$%^&*()-_=+[]{}|;:,.<>?/";

    let mut charset = String::from(lowercase);
    if include_uppercase {
        charset.push_str(uppercase);
    }
    if include_numbers {
        charset.push_str(numbers);
    }
    if include_symbols {
        charset.push_str(symbols);
    }

    let charset_vec: Vec<char> = charset.chars().collect();

    for i in 1..=number_of_passwords {
        let password = generate_password(password_length, &charset_vec);
        println!("Password {}: {}", i, password);
    }
}

fn generate_password(length: usize, charset: &Vec<char>) -> String {
    let mut rng = rand::thread_rng();
    let mut password = String::new();

    for _ in 0..length {
        let idx = rng.gen_range(0..charset.len());
        password.push(charset[idx]);
    }

    password
}
