#[derive(Debug)]
#[deprecated]
pub struct Account<'a> {
    // id: &'static str,
    id: &'a str,
    blance: f32,
    interest: f32,
    rate: f32,
}

impl<'a> Account<'a>{
    pub fn new(id: &'a str, blance: f32, interest: f32, rate: f32) -> Account {
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
