fn main() {
    println!("Hello, world!");
    let my_number = 8;
    print_number(my_number);
    print_number(my_number);

    let my_country = "Austria".to_string();

    prints_string(my_country.clone());
    prints_string(my_country.clone());
    prints_string(my_country);

    let my_number;
}

fn prints_string(input: String) -> String {
    println!("{input}");
    input
}

fn print_number(number: i32) {
    println!("{}", number);
}
