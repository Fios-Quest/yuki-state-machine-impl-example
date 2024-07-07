use super::hangry::Hangry;

pub struct Mischievous<A> {
    animal: A,
}

impl<A> Mischievous<A> {
    pub fn new(animal: A) -> Self {
        Mischievous { animal }
    }

    pub fn forget_to_feed(self) -> Hangry<A> {
        Hangry::new(self.animal)
    }

    pub fn describe(&self) -> String {
        "Trying to break into a wardrobe by pulling on exposed clothing".to_string()
    }
}
