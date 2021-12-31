use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() -> std::io::Result<()> {
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
            Some(index) if index == 4 => break Ok(()),
            Some(index) => println!("User selected item : {}", items[index]),
            None => println!("User did not select anything"),
        }
    }
}
