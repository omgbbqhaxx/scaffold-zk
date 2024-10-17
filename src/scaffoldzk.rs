use std::io;
//Get address input
pub fn get_address(placeholder: &str) -> String {
    println!("Your input is here : {}", placeholder);
    read_address() 
}

fn is_valid_address(address: &str) -> bool {
    // Ethereum adresi "0x" ile başlamalı ve 42 karakter uzunluğunda olmalı...
    address.starts_with("0x") && address.len() == 42 && address.chars().all(|c| c.is_digit(16) || c == 'x')
}

fn read_address() -> String {
    loop {
        let mut address = String::new();

        std::io::stdin()
            .read_line(&mut address)
            .expect("Failed to read from stdin");

        address = address.trim().to_string();

        if !is_valid_address(&address) {
            println!("Please enter a valid Ethereum address (42 characters starting with '0x')");
            continue;
        }

        return address;
    }
}

//Get string input

pub fn get_string(placeholder: &str, max_length: usize) -> String {
    println!("Your input is here : {}", placeholder);
    read_string(max_length) 
}

fn is_valid_length(input: &str, max_len: usize) -> bool {
    // String'in maksimum uzunluk sınırını kontrol eder
    input.len() <= max_len
}

fn read_string(max_length: usize) -> String {
    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        input = input.trim().to_string();

        // Sadece maksimum uzunluk kontrolü yapılır
        if !is_valid_length(&input, max_length) {
            println!("Please enter a string with a maximum of {} characters. You entered {} characters.", max_length, input.len());
            continue;
        }

        return input;
    }
}

//Get integer input with digit

pub fn get_integer(prompt: &str, max_value: i32) -> i32 {
    println!("{}", prompt);
    read_integer(max_value)
}

fn is_valid_integer(input: &str, max_value: i32) -> bool {
    match input.parse::<i32>() {
        Ok(value) => value <= max_value,
        Err(_) => false,
    }
}

fn read_integer(max_value: i32) -> i32 {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        input = input.trim().to_string();

        if !is_valid_integer(&input, max_value) {
            println!("Please enter a valid integer less than or equal to {}. You entered: {}", max_value, input);
            continue;
        }

        // Girdi geçerli bir integer olduğunda döndür
        return input.parse().expect("Failed to parse integer");
    }
}


//Get ethereum public key and check is valid

//Get ethereum private key and check is valid


//Get Ask and Get questions.

pub fn ask_question(question: &str, answers: &[&str]) -> char {
    println!("{}", question);
    for (i, answer) in answers.iter().enumerate() {
        println!("{}. {}", (b'a' + i as u8) as char, answer);
    }

    read_answer()
}

fn is_valid_answer(answer: char) -> bool {
    answer == 'a' || answer == 'b' || answer == 'c'
}

fn read_answer() -> char {
    loop {
        let mut answer = String::new();

        std::io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read from stdin");

        answer = answer.trim().to_string();
        if answer.len() != 1 {
            println!("Please enter a valid answer (a, b or c)");
            continue;
        }

        let c = answer.chars().next().unwrap();
        if !is_valid_answer(c) {
            println!("Please enter a valid answer (a, b or c)");
            continue;
        }

        return c;
    }
}