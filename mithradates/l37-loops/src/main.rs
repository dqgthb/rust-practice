fn main() {
    let mut counter = 0;
    let mut counter2 = 0;
    'first_loop: loop {
        counter += 1;
        println!("The counter is now: {}", counter);

        'second_loop: loop {
            counter2 += 1;

            println!("The counter is: {}", counter);
            if counter2 == 5 {
                break 'first_loop;
            }
        }
    }

    let mut counter = 0;
    while counter != 5 {
        counter += 1;
    }

    for number in 0..=3 {
        println!("{number}");
    }
}
