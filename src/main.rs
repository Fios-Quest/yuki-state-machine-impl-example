mod cat;
mod state;

use cat::Cat;

fn main() {
    let mischievous_yuki = Cat::new("Yuki".to_string());
    println!("{}", mischievous_yuki.describe());
    println!();

    let hangry_yuki = mischievous_yuki.forget_to_feed();
    println!("{}", hangry_yuki.describe());
    println!();

    let sleepy_yuki = hangry_yuki.feed();
    println!("{}", sleepy_yuki.describe());
    println!();

    let mischievous_yuki = sleepy_yuki.sleep();
    println!("{}", mischievous_yuki.describe());
    println!();
}
