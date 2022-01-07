use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select, Sort};
use entries::{print_all, Entry};
use permutation::Permutation;
mod entries;

// screen to create a new entry
fn create_entry(term: &Term) -> Entry {
    let prompt = String::from("New Entry ->");
    println!("{}", prompt);

    let mut entry = Entry {
        name: String::from(""),
        amount: 0f32,
    };

    loop {
        entry.name = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Name")
            .with_initial_text(entry.name.to_owned())
            .default(entry.name)
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.is_empty() {
                    Err("Name cannot be empty")
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .unwrap();

        entry.amount = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Amount")
            .with_initial_text(if entry.amount > 0f32 {
                entry.amount.to_string()
            } else {
                String::from("")
            })
            .default(entry.amount)
            .validate_with(|input: &f32| -> Result<(), &str> {
                if input > &0f32 {
                    Ok(())
                } else {
                    Err("Amount must be a valid number greater than 0")
                }
            })
            .interact_text()
            .unwrap();

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Create new entry? [Y/n]")
            .default(true)
            .interact()
            .unwrap()
        {
            return entry;
        } else {
            term.clear_last_lines(3).unwrap();
        }
    }
}

// screen to edit a single entry
fn edit_entry(term: &Term, entry: Entry) -> Entry {
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
            .interact()
            .unwrap();

        match items[select_mode] {
            "Edit Name" => {
                // NAME
                modified_entry.name = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Name")
                    .with_initial_text(modified_entry.name.to_string())
                    .default(modified_entry.name)
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if input.is_empty() {
                            Err("Name cannot be empty")
                        } else {
                            Ok(())
                        }
                    })
                    .interact_text()
                    .unwrap();
            }
            "Edit Amount" => {
                // AMOUNT
                modified_entry.amount = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Amount")
                    .with_initial_text(modified_entry.amount.to_string())
                    .default(modified_entry.amount)
                    .validate_with(|input: &f32| -> Result<(), &str> {
                        if input > &0f32 {
                            Ok(())
                        } else {
                            Err("Amount must be a valid number greater than 0")
                        }
                    })
                    .interact_text()
                    .unwrap();
            }
            "Save and Return" => return modified_entry, // SAVE AND RETURN
            "Return" => return entry,                   // RETURN
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
            entries[index] = edit_entry(term, entry);
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
    let items = ["New", "Edit", "Rearrange", "Main Menu"];

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
            "New" => {
                entries.push(create_entry(term));
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
