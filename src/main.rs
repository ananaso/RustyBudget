use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

mod entries;
use entries::{print_all, Entry};

fn handle_result(result: std::io::Result<()>) {
    match result {
        Ok(_v) => (),
        Err(_e) => println!("Error: {}", _e),
    }
}

fn entry_menu(term: &Term, title: &str, entries: &mut Vec<Entry>) -> std::io::Result<()> {
    let items = ["Rearrange", "Edit", "Main Menu"];

    println!("{} ->", title);
    print_all(entries);

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(term)?;

        match selection {
            Some(index) if index == items.iter().position(|&x| x == "Main Menu").unwrap() => {
                break Ok(())
            }
            Some(index) => println!("User selected item : {}", items[index]),
            None => println!("User did not select anything"),
        }
    }
}

fn main() -> std::io::Result<()> {
    let term = &Term::stderr();

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
        handle_result(term.clear_screen());

        let items = ["Expenses", "Savings", "Free Spending", "Quit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(last_selected)
            .interact_on_opt(term)?;

        last_selected = selection.unwrap();

        match selection {
            Some(index) if index == items.iter().position(|&x| x == "Expenses").unwrap() => {
                handle_result(entry_menu(term, "Expenses", &mut expenses));
            }
            Some(index) if index == items.iter().position(|&x| x == "Savings").unwrap() => {
                handle_result(entry_menu(term, "Savings", &mut savings));
            }
            Some(index) if index == items.iter().position(|&x| x == "Quit").unwrap() => {
                break Ok(())
            }
            Some(index) => println!("User selected item : {}", items[index]),
            None => println!("User did not select anything"),
        }
    }
}
