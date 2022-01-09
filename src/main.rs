use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select, Sort};
use entries::{print_all, Entry};
use permutation::Permutation;
mod entries;

// screen to create a new entry
fn create_entry(term: &Term) -> std::io::Result<Entry> {
    let prompt = String::from("New Entry ->");
    println!("{}", prompt);

    let mut entry = Entry::default();

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
            .interact_text()?;

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
            .interact_text()?;

        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Create new entry?")
            .default(true)
            .interact_opt()?;

        match confirm {
            Some(answer) => {
                if answer {
                    break;
                } else {
                    term.clear_last_lines(3)?;
                }
            }
            None => {
                entry = Entry::default();
                break;
            }
        }
    }

    Ok(entry)
}

// screen to edit a single entry
fn edit_entry(term: &Term, entry: Entry) -> std::io::Result<Entry> {
    let mut prompt = String::from("Editing ");
    prompt.push_str(entry.to_string().as_str());
    prompt.push_str(" -> ");
    let items = ["Name", "Amount", "Save and Return", "Return"];
    let mut modified_entry = entry.clone();

    loop {
        term.clear_screen()?;
        let mut prompt_mod = prompt.to_owned();
        prompt_mod.push_str(modified_entry.to_string().as_str());

        let select_mode = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt_mod)
            .items(&items)
            .default(0)
            .interact()?;

        match items[select_mode] {
            "Name" => {
                // NAME
                modified_entry.name = Input::with_theme(&ColorfulTheme::default())
                    .with_initial_text(modified_entry.name.to_string())
                    .default(modified_entry.name)
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if input.is_empty() {
                            Err("Name cannot be empty")
                        } else {
                            Ok(())
                        }
                    })
                    .interact_text()?;
            }
            "Amount" => {
                // AMOUNT
                modified_entry.amount = Input::with_theme(&ColorfulTheme::default())
                    .with_initial_text(modified_entry.amount.to_string())
                    .default(modified_entry.amount)
                    .validate_with(|input: &f32| -> Result<(), &str> {
                        if input > &0f32 {
                            Ok(())
                        } else {
                            Err("Amount must be a valid number greater than 0")
                        }
                    })
                    .interact_text()?;
            }
            "Save and Return" => return Ok(modified_entry), // SAVE AND RETURN
            "Return" => return Ok(entry),                   // RETURN
            &_ => println!("User selected unhandled item"),
        }
    }
}

// screen to select which entry to edit
fn edit_menu(term: &Term, mut entries: Vec<Entry>) -> std::io::Result<Vec<Entry>> {
    let entry_strings: Vec<String> = entries.iter().map(|entry| entry.to_string()).collect();

    term.clear_screen()?;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select entry to edit")
        .items(&entry_strings)
        .default(0)
        .interact_opt()?;

    if let Some(index) = selection {
        entries[index] = edit_entry(term, entries[index].clone())?;
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

    term.clear_screen()?;

    let ordered = Sort::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&entry_strings)
        .interact_opt()?;

    if let Some(new_positions) = ordered {
        let permutation = Permutation::from_vec(new_positions);
        entries = permutation.apply_slice(entries);
    }

    Ok(entries)
}

// screen to view and manage a class of entries
fn entry_menu(term: &Term, title: &str, mut entries: Vec<Entry>) -> std::io::Result<Vec<Entry>> {
    let items = ["New", "Edit", "Rearrange", "Main Menu"];

    loop {
        term.clear_screen()?;
        println!("{} ->", title);
        print_all(&entries);

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_opt()?;

        match selection {
            Some(index) => {
                let selected_menu_item = items[index];
                match selected_menu_item {
                    "Main Menu" => break,
                    "Rearrange" => {
                        entries = rearrange_menu(term, title, entries)?;
                    }
                    "Edit" => {
                        entries = edit_menu(term, entries)?;
                    }
                    "New" => {
                        let new_entry = create_entry(term)?;
                        if !new_entry.is_default() {
                            entries.push(new_entry);
                        }
                    }
                    &_ => panic!("Selected invalid menu item"),
                }
            }

            None => break,
        }
    }

    Ok(entries)
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
        term.clear_screen()?;

        let items = ["Expenses", "Savings", "Quit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(term)?;

        match selection {
            Some(index) => {
                let selected_menu_item = items[index];
                match selected_menu_item {
                    "Expenses" => {
                        expenses = entry_menu(term, "Expenses", expenses)?;
                    }
                    "Savings" => {
                        savings = entry_menu(term, "Savings", savings)?;
                    }
                    "Quit" => break Ok(()),
                    &_ => panic!("Selected invalid menu item"),
                }
            }
            None => println!("User selected unhandled item"),
        }
    }
}
