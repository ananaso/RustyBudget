use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use expenses::{print_expenses, Expense};
mod expenses;

fn main() -> std::io::Result<()> {
    let mut expenses = vec![
        Expense {
            name: String::from("apples"),
            amount: 25f32,
        },
        Expense {
            name: String::from("oranges"),
            amount: 32.50f32,
        },
    ];

    loop {
        let items = [
            "View Expenses",
            "View Savings",
            "View Free Spending",
            "Help",
            "Quit",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        match selection {
            Some(index) if index == items.iter().position(|&x| x == "View Expenses").unwrap() => {
                print_expenses(&mut expenses)
            }
            Some(index) if index == items.iter().position(|&x| x == "Quit").unwrap() => {
                break Ok(())
            }
            Some(index) => println!("User selected item : {}", items[index]),
            None => println!("User did not select anything"),
        }
    }
}
