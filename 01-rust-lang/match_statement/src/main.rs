use crate::Suit::{Heart, Club, Diamond, Spade};
fn main() {
    print_choice(Heart);
    print_choice(Club);
    print_choice(Spade);
    print_choice(Diamond);

    country(44);
    country(34);
    country(125);
    country(-14);
}

#[allow(dead_code)]
enum Suit {
    Heart,
    Spade,
    Club,
    Diamond
}

// match statement can return a result
// ranges are allowed 
// example: based on a country code a country name will be returned
fn country(code: i32) {
    let country = match code {
        44 => "UK",
        34 => "Spain",
        49 => "Germany",
        1..=999 => "unknown",
        _ => "invalid"
    };
    println!("Country is {}", country);
}

#[allow(dead_code)]
fn print_choice(choice: Suit) {
    // match statements are similar to when or switch stetements
    // in other languages
    match choice {
        Heart => {println!("\u{2665}")}
        Spade => {println!("\u{2660}")}
        Club => {println!("\u{2663}")}
        Diamond => {println!("\u{2666}")}
    } 
}
