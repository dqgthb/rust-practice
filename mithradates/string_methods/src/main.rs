fn main() {
    println!("Hello, world!");

    let mut my_name = "".to_string();
    println!(
        "Length is {} and capacity is: {}",
        my_name.len(),
        my_name.capacity()
    );
    my_name.push_str("David");
    println!(
        "Length is {} and capacity is: {}",
        my_name.len(),
        my_name.capacity()
    );
    my_name.push_str("and I live in Seoul");
    println!(
        "Length is {} and capacity is: {}",
        my_name.len(),
        my_name.capacity()
    );
    my_name.push_str("and I live in Seoul haha");
    println!(
        "Length is {} and capacity is: {}",
        my_name.len(),
        my_name.capacity()
    );
}
