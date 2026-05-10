use std:: io; 
use std:: collections :: HashMap;

struct Bill{
    name: String,
    amount: f64,
}
fn show_menu() {
    println!("\n--- Bill Manager ---");
    println!("1. Add or Edit Bill?");
    println!("2. View Bills?");
    println!("3. Remove Bill?");
    println!("4. Quit!");
    println!("Enter selection: ");
}

//  function to get input from the user and return it as a String.
fn get_input() -> String {
    // using String as the key
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_owned() // .trim() removes the newline character (\n)
}

fn main() {
    // // vecotor to store the bills
    // let mut bills: Vec<Bill> = Vec::new();
    let mut bills: HashMap<String, Bill> = HashMap::new();

    loop {
        show_menu();
        let input = get_input();

        // matching on the user input string
        match input.as_str() {
            "1" => {
                println!("Enter bill name (or type 'back' to cancel):");
                let name = get_input();
                if name == "back" {continue; }
                
                println!("Enter amount:");
                let amount_str = get_input();
                // We attempt to parse the string into a number (f64)
                let amount: f64 = match amount_str.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number!");
                        continue;
                    }
                };
                if amount_str == "back" { continue; }

                let bill = Bill { name: name.clone(), amount };
                
                if bills.insert(name, bill).is_some() {
                    println!("Bill updated!");
                } else {
                    println!("Bill added!");
                }
            }
            "2" => {
                println!("\n--- Current Bills ---");
                //printing  each bill
                if bills.is_empty(){
                    println!("No bills found, your list is empty")
                }else{
                    for bill in bills.values() {
                    println!("Name: {}, Amount: ${:.2}", bill.name, bill.amount);
                }
                }
            }
            "3" => {
                println!("Enter the name of the Bill to remove: ");
                let name = get_input();

                if bills.remove(&name).is_some() {
                    println!("Bill removed.");
                } else {
                    println!("Bill not found.");
                }
            }
            "4" => {
                println!("Thank you Goodbye!");
                break;
            }
            _ => println!("Invalid selection, try again."),
        }
    }
}
