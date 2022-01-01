use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

mod entries;
use entries::{expenses, savings, Entry};

fn main() -> std::io::Result<()> {
    let mut expenses = vec![
        Entry {
            name: String::from("apples"),
            amount: 25f32,
        },
        Entry {
            name: String::from("oranges"),
            amount: 32.50f32,
        },
    ];

    let mut savings = vec![
        Entry {
            name: String::from("Emergency Fund"),
            amount: 442f32,
        },
        Entry {
            name: String::from("Car"),
            amount: 450f32,
        },
    ];

    let mut last_selected = 0;

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
            .default(last_selected)
            .interact_on_opt(&Term::stderr())?;

        last_selected = selection.unwrap();

        match selection {
            Some(index) if index == items.iter().position(|&x| x == "View Expenses").unwrap() => {
                expenses::print_all(&mut expenses)
            }
            Some(index) if index == items.iter().position(|&x| x == "View Savings").unwrap() => {
                savings::print_all(&mut savings)
            }
            Some(index) if index == items.iter().position(|&x| x == "Quit").unwrap() => {
                break Ok(())
            }
            Some(index) => println!("User selected item : {}", items[index]),
            None => println!("User did not select anything"),
        }
    }
}
