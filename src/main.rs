use console::Term;
use dialoguer::{theme::ColorfulTheme, Select, Sort};
use entries::{print_all, Entry};
use permutation::Permutation;
mod entries;

fn rearrange_menu(
    term: &Term,
    title: &str,
    mut entries: Vec<Entry>,
) -> std::io::Result<Vec<Entry>> {
    let mut prompt = String::from("Rearrange ");
    prompt.push_str(title);
    let entry_names: Vec<String> = entries
        .iter()
        .map(|entry| [entry.name.to_string(), entry.amount.to_string()].join(": "))
        .collect();

    term.clear_screen().unwrap();

    let ordered = Sort::new()
        .with_prompt(prompt)
        .items(&entry_names)
        .interact_on_opt(term)?;

    match ordered {
        Some(new_positions) => {
            let permutation = Permutation::from_vec(new_positions);
            entries = permutation.apply_slice(entries);
        }
        None => println!("Entries not reordered"),
    }

    Ok(entries)
}

fn entry_menu(term: &Term, title: &str, mut entries: Vec<Entry>) -> std::io::Result<Vec<Entry>> {
    let items = ["Rearrange", "Edit", "Main Menu"];

    loop {
        term.clear_screen().unwrap();
        println!("{} ->", title);
        print_all(&entries);

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(term)?;

        match selection {
            Some(index) if index == items.iter().position(|&x| x == "Main Menu").unwrap() => {
                break Ok(entries)
            }
            Some(index) if index == items.iter().position(|&x| x == "Rearrange").unwrap() => {
                entries = rearrange_menu(term, title, entries).unwrap();
            }
            Some(index) => println!("User selected item : {}", items[index]),
            None => println!("User did not select anything"),
        }
    }
}

fn main() -> std::io::Result<()> {
    let term = &Term::stdout();

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
        term.clear_screen().unwrap();

        let items = ["Expenses", "Savings", "Free Spending", "Quit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(last_selected)
            .interact_on_opt(term)?;

        last_selected = selection.unwrap();

        match selection {
            Some(index) if index == items.iter().position(|&x| x == "Expenses").unwrap() => {
                expenses = entry_menu(term, "Expenses", expenses).unwrap();
            }
            Some(index) if index == items.iter().position(|&x| x == "Savings").unwrap() => {
                savings = entry_menu(term, "Savings", savings).unwrap();
            }
            Some(index) if index == items.iter().position(|&x| x == "Quit").unwrap() => {
                break Ok(())
            }
            Some(index) => println!("User selected item : {}", items[index]),
            None => println!("User did not select anything"),
        }
    }
}
