use dialoguer::{
    Select,
    theme::ColorfulTheme
};
use console::Term;

fn main() -> std::io::Result<()> {
    let items = ["View Expenses", "View Savings", "View Free Spending", "Help"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

        match selection {
            Some(index) => println!("User selected item : {}", items[index]),
            None => println!("User did not select anything")
        }

        Ok(())
}
