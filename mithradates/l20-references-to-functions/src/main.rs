fn print_country(country_name: &String) {
    println!("My country is {}", country_name);
}

fn add_is_great(mut country_name: String) -> String {
    // take by value, declare as mutable
    country_name.push_str(" is great!");
    println!("Now it says: {}", country_name);
    country_name
}

fn main() {
    let country = "South Korea".to_string();
    print_country(&country);
    print_country(&country);
    print_country(&country);

    let mut my_country = "Canada".to_string();
    my_country = add_is_great(my_country);
    println!("{my_country}");
}
