use super::eepy::Eepy;

pub struct Hangry<A> {
    animal: A,
}

impl<A> Hangry<A> {
    pub fn new(animal: A) -> Self {
        Hangry { animal }
    }

    pub fn feed(self) -> Eepy<A> {
        Eepy::new(self.animal)
    }

    pub fn describe(&self) -> String {
        "Being loud doesn't get what they want, they choose violence and attack!".to_string()
    }
}
