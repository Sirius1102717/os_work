
#[derive(Debug)]
pub struct Account{
    // id: &'static str,
    id: String,
    blance: f32,
    interest: f32,
    rate: f32,
}

impl Account{
    pub fn new(id: &String, blance: f32, interest: f32, rate: f32) -> Account {
        Account {
            id: id.to_string(),
            blance,
            interest,
            rate,
        }
    }
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn blance(&self) -> f32 {
        self.blance
    }

    pub fn interest(&self) -> f32 {
        self.interest
    }

    pub fn deposit(&mut self, amount: f32) {
        self.blance += amount
    }
    pub fn withdraw(&mut self, amount: f32) {
        self.blance -= amount
    }

    pub fn calculator_interest(&mut self) {
        self.interest += self.blance * self.rate
    }


    
}
