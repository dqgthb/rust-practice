fn main() {
    println!("Hello, world!");

    let my_number = {
        let x = loop_then_return(43);
        x
    };

    println!("{my_number}");
}

fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0 {
            break;
        }
    }
    counter
}
