use std::io;
fn main() {
    loop {
        let mut response = String::new();

        println!(
            "
(NEW) - Create new deck
(EDIT) - Edit existing deck
(EXIT) - Leave Lost Memory
            "
        );

        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        match response.to_lowercase().trim() {
            "new" => println!("Implement creating a new deck"),
            "edit" => println!("Implement edit a deck functionality"),
            "exit" => break,
            x => println!("{} is not a options", x),
        }
    }
}
