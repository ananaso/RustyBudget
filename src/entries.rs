#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub amount: f32,
}

pub fn print_all(entries: &Vec<Entry>) {
    for entry in entries {
        println!("\t{}: {}", entry.name, entry.amount);
    }
}
