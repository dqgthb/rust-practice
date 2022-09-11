// & immutable reference / shared reference
// &mut mutable reference / unique reference

fn main() {
    let country = "south korea";
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country_ref, country);
}
