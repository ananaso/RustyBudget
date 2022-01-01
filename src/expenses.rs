pub struct Expense {
    pub name: String,
    pub amount: f32,
}

pub fn print_expenses(expenses: &mut Vec<Expense>) {
    println!("Expenses ->");
    for expense in expenses {
        println!("\t{}: {}", expense.name, expense.amount)
    }
}
