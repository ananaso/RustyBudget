use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select, Sort};
use entries::{print_all, Entry};
use permutation::Permutation;
mod entries;

// screen to edit a single entry
fn edit_entry(term: &Term, entry: Entry) -> std::io::Result<Entry> {
    let mut prompt = String::from("Editing ");
    prompt.push_str(entry.to_string().as_str());
    let items = ["Edit Name", "Edit Amount", "Save and Return", "Return"];
    let mut modified_entry = entry.clone();

    loop {
        term.clear_screen().unwrap();
        println!("{} -> {}", prompt, modified_entry.to_string());

        let select_mode = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(term)?;

        match items[select_mode.unwrap()] {
            "Edit Name" => {
                // NAME
                modified_entry.name = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Name")
                    .with_initial_text(modified_entry.name.to_string())
                    .interact_text_on(term)?;
            }
            "Edit Amount" => {
                // AMOUNT
                modified_entry.amount = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Amount")
                    .with_initial_text(modified_entry.amount.to_string())
                    .interact_text_on(term)?;
            }
            "Save and Return" => break Ok(modified_entry), // SAVE AND RETURN
            "Return" => break Ok(entry),                   // RETURN
            &_ => println!("User selected unhandled item"),
        }
    }
}

// screen to select which entry to edit
fn edit_menu(term: &Term, mut entries: Vec<Entry>) -> std::io::Result<Vec<Entry>> {
    let entry_strings: Vec<String> = entries.iter().map(|entry| entry.to_string()).collect();

    term.clear_screen().unwrap();

    let select_to_edit = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select which entry to edit")
        .items(&entry_strings)
        .default(0)
        .interact_on_opt(term)?;

    match select_to_edit {
        Some(index) => {
            let entry = entries[index].clone();
            entries[index] = edit_entry(term, entry).unwrap();
        }
        None => println!("User didn't select anything"),
    }

    Ok(entries)
}

// screen to rearrange the order which entries are listed in
fn rearrange_menu(
    term: &Term,
    title: &str,
    mut entries: Vec<Entry>,
) -> std::io::Result<Vec<Entry>> {
    let mut prompt = String::from("Rearrange ");
    prompt.push_str(title);
    let entry_strings: Vec<String> = entries.iter().map(|entry| entry.to_string()).collect();

    term.clear_screen().unwrap();

    let ordered = Sort::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&entry_strings)
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

// screen to view and manage a class of entries
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

        match items[selection.unwrap()] {
            "Main Menu" => break Ok(entries),
            "Rearrange" => {
                entries = rearrange_menu(term, title, entries).unwrap();
            }
            "Edit" => {
                entries = edit_menu(term, entries).unwrap();
            }
            &_ => println!("User selected unhandled item"),
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

    loop {
        term.clear_screen().unwrap();

        let items = ["Expenses", "Savings", "Free Spending", "Quit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(term)?;

        match items[selection.unwrap()] {
            "Expenses" => {
                expenses = entry_menu(term, "Expenses", expenses).unwrap();
            }
            "Savings" => {
                savings = entry_menu(term, "Savings", savings).unwrap();
            }
            "Quit" => break Ok(()),
            &_ => println!("User selected unhandled item"),
        }
    }
}
