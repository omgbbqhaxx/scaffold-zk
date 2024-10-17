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