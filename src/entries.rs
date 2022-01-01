pub struct Entry {
    pub name: String,
    pub amount: f32,
}

fn print_all(entries: &mut Vec<Entry>) {
    for entry in entries {
        println!("\t{}: {}", entry.name, entry.amount);
    }
}

pub mod expenses {
    pub fn print_all(expenses: &mut Vec<super::Entry>) {
        println!("Expenses ->");
        super::print_all(expenses);
    }
}

pub mod savings {
    pub fn print_all(savings: &mut Vec<super::Entry>) {
        println!("Savings ->");
        super::print_all(savings);
    }
}
