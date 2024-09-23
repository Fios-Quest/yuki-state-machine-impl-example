use crate::state::mischievous::Mischievous;

pub struct Cat {
    name: String,
}

impl Cat {
    pub fn new(name: String) -> Mischievous<Cat> {
        Mischievous::new(Self { name })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
