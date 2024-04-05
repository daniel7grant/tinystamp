use tinystamp::Datetime;

#[cfg(feature = "std")]
fn main() {
    println!("{}", Datetime::now());
}
