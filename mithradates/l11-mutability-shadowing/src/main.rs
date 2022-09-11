fn main() {
    // String
    // &str ref str "string slice"
    let my_name = "David".to_string();
    let other_name = String::from("David2");
    let mut my_other_name = "David3".to_string();
    my_other_name.push('!');
    println!("{my_name:?}");
    println!("{my_other_name}");
}
