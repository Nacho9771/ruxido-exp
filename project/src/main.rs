
use std::io;

fn build_expense(item: String, amount: f64) -> Expense {
    struct Expense {
        id: usize,
        item,
        amount,
    }
}


struct IdManager {
    next_id: usize,
}

fn intro_sequence() {

}

fn main() {
   intro_sequence();

    let mut item = String::new();
    io::stdin()
        .read_line(&mut item)
        .expect("Failed to read line");

    let item: &str = item.trim().expect("Please enter the item name:");
}