use tinystamp::Datetime;

fn main() {
    #[cfg(feature = "std")]
    println!("{}", Datetime::now());
}
