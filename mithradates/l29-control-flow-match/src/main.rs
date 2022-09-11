fn main() {
    let my_number = 5;
    match my_number {
        0 => println!("It's a zero"),
        1 => println!("It's a one"),
        _ => println!("It's a different number"), // "I don't care" "anything else"
    };

    let second_number = match my_number {
        0 => 23,
        1 => 65,
        _ => 0,
    };

    println!("{second_number}");

    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's not very nice today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", _) => println!("Cloudy and something else"),
        _ => println!("Not sure what the weather is"),
    }

    let children = 5;
    let married = true;

    match (children, married) {
        (_, m) if m == false => println!("Not married with {children} children"),
        (c, m) if c == 0 && m => println!("Married but with no children"),
        _ => println!("Some other type of marriage and children combination"),
    }

    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);
    match_number(10);
    match_number(11);
    match_number(12);
}

fn match_colours(rgb: (i32, i32, i32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Every colour has at least 10"),
    }
}

fn match_number(input: i32) {
    match input {
        number @ 0..=10 => println!("It's between 0 and 10. It's the number {number}"),
        _ => println!("It's greater than ten"),
    }
}
