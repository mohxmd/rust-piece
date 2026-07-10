// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(String, f64),
    Vip(String, f64),
    Standard(f64),
}

fn main() {
    let backstage_ticket = Ticket::Backstage("Nami".to_owned(), 12500.0);
    let vip_ticket = Ticket::Vip(String::from("Vivi"), 6000.0);
    let standard_ticket = Ticket::Standard(1500.0);

    let tickets = vec![backstage_ticket, vip_ticket, standard_ticket];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(name, price) => {
                println!("Backstage ticket");
                println!("Holder: {}", name);
                println!("Price: ₹{}", price);
            }

            Ticket::Vip(name, price) => {
                println!("VIP ticket");
                println!("Holder: {}", name);
                println!("Price: ₹{}", price);
            }

            Ticket::Standard(price) => {
                println!("Standard ticket");
                println!("Price: ₹{}", price);
            }
        }
        println!("--------------------");
    }
}
