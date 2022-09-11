// struct
use std::mem::size_of_val;

// unit struct
struct FileDirectory;

#[derive(Debug)]
struct Colour(u8, u8, u8);

// named struct
#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn main() {
    println!("Hello, world!");
    let x = FileDirectory;

    println!("The size is {}", std::mem::size_of_val(&x));
    let my_colour = Colour(20, 50, 100);
    println!("The size is {}", std::mem::size_of_val(&my_colour));
    println!("{my_colour:?}");

    let canada = Country {
        population: 35_000_000,
        capital: "Ottawa".to_string(),
        leader_name: "Justin Trudeau".to_string(),
    };

    println!("The population is: {}", canada.population);
    println!("The population is: {}", canada.leader_name);
    println!("The population is: {}", canada.capital);

    println!("The country is: {:?}", canada);
    println!("The country is: {:#?}", canada);

    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader = "Justin Trudeau".to_string();

    let my_country = Country {
        population: population,
        capital: capital,
        leader_name: leader,
    };

    println!("Country is {} bytes in size", size_of_val(&my_colour));
    println!("Country is {} bytes in size", size_of_val(&my_country));
}
