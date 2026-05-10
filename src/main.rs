use std:: io; 

struct Bill{
    name: String,
    amount: f64,
}
fn show_menu() {
    println!("\n--- Bill Manager ---");
    println!("1. Add Bill");
    println!("2. View Bills");
    println!("3. Quit");
    println!("Enter selection: ");
}

//  function to get input from the user and return it as a String.
fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_owned() // .trim() removes the newline character (\n)
}

fn main() {
    // vecotor to store the bills
    let mut bills: Vec<Bill> = Vec::new();

    loop {
        show_menu();
        let input = get_input();

        // matching on the user input string
        match input.as_str() {
            "1" => {
                println!("Enter bill name:");
                let name = get_input();
                
                println!("Enter amount:");
                let amount_str = get_input();
                // We attempt to parse the string into a number (f64)
                let amount: f64 = amount_str.parse().expect("Please enter a valid number");

                let bill = Bill { name, amount };
                bills.push(bill); 
                println!("Bill added!");
            }
            "2" => {
                println!("\n--- Current Bills ---");
                //printing  each bill
                for bill in &bills {
                    println!("Name: {}, Amount: ${:.2}", bill.name, bill.amount);
                }
            }
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid selection, try again."),
        }
    }
}
