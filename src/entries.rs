use std::fmt;

#[derive(Clone, Default)]
pub struct Entry {
    pub name: String,
    pub amount: f32,
}

impl Entry {
    pub fn is_default(&self) -> bool {
        self.name == "" && self.amount == 0f32
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.amount)
    }
}

pub fn print_all(entries: &Vec<Entry>) {
    for entry in entries {
        println!("\t{}: {}", entry.name, entry.amount);
    }
}
