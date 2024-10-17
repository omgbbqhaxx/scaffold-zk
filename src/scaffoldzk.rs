//Get address input
pub fn get_address_input(input: &str) -> String {
    println!("Your input is here : {}", input);
    read_address() // Adresi almak için `read_address` fonksiyonunu çağırıyoruz
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

//Get integer input with digit

//Get ethereum public key and check is valid

//Get ethereum private key and check is valid