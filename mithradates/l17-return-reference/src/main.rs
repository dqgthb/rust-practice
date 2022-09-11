fn main() {
    println!("Hello, world!");

    let country = String::from("South Korea");
    let ref_one = &country;
    let ref_two = &country;

    println!("Country is {}", ref_one);

    let my_country = return_it();
}

fn return_it() -> &String {
    let country = String::from("South Korea?");
    &country // does not work
}
