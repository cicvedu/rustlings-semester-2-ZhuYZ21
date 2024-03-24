fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    let uppercased_data = string_uppercase(data);

    println!("{}", uppercased_data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership and return a new uppercase String
fn string_uppercase(data: String) -> String {
    data.to_uppercase()
}
