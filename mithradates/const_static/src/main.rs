const HIGH_SCORE: i32 = 20;
// static lifetime (can live until the end of the program execution.)
static mut LOW_SCORE: i32 = 0;

fn print_high_score() {
    println!("The high score is {}", HIGH_SCORE);
}

fn main() {
    println!("Hello, world!");

    let x = 8;
    let my_name = "David"; // &'static str
}
