/*
fn main() {
    let animal = "Dog";

    if animal == "Dog" {
        println!("Woof!");
    } else if animal == "Cat" {
        println!("Meow!");
    } else {
        println!("Unknown animal");
    }
}
*/

fn main() {
    let animal = "Cat";

    match animal {
        "Dog" => println!("Woof!"),
        "Cat" => println!("Meow!"),
        _ => println!("Unknown animal")
    }
}