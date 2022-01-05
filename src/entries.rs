#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub amount: f32,
}

impl Entry {
    pub fn from(entry: &Entry) -> Self {
        Self {
            name: entry.name.clone(),
            amount: entry.amount,
        }
    }
}

pub fn print_all(entries: &Vec<Entry>) {
    for entry in entries {
        println!("\t{}: {}", entry.name, entry.amount);
    }
}
