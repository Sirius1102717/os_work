pub struct Account {
    id: &'static str,
    blance: f32,
    interest: f32,
    rate: f32,
}

impl Account {
    pub fn new(id: &'static str, blance: f32, interest: f32, rate: f32) -> Account {
        Account {
            id,
            blance,
            interest,
            rate,
        }
    }
    pub fn id(&self) -> &str {
        self.id
    }

    pub fn blance(&self) -> f32 {
        self.blance
    }

    pub fn deposit(&mut self, ammount: f32) {
        self.blance += ammount
    }
    pub fn withdraw(&mut self, ammount: f32) {
        self.blance -= ammount
    }

    pub fn calculator_interest(&mut self) {
        self.interest = self.blance * self.rate
    }
}
