use super::mischievous::Mischievous;

pub struct Eepy<A> {
    animal: A,
}

impl<A> Eepy<A> {
    pub fn new(animal: A) -> Self {
        Eepy { animal }
    }

    pub fn sleep(self) -> Mischievous<A> {
        Mischievous::new(self.animal)
    }

    pub fn describe(&self) -> String {
        "Look at the precious baby sleeping 😍".to_string()
    }
}
