// pub use os_work::account::Account;
// use os_work::account::Account;
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use std::thread::spawn;
// 
// lazy_static! {
    // static ref ACCOUNTS_INDEX: Mutex<HashMap<&'static str, usize>> = {
        // let m = HashMap::new();
        // Mutex::new(m)
    // };
// }
// 
// pub struct Bank {
    // accounts: Vec<Account>,
    // accounts_index: HashMap<&'static str, usize>,
// }
// impl Bank {
    // pub fn new() -> Self {
        // Bank {
            // accounts: Vec::new(),
            // accounts_index: HashMap::new(),
        // }
    // }
// 
    // pub fn add_account(&mut self, id: &'static str, blance: f32, interest: f32, rate: f32) {
        // let account: Account = Account::new(id, blance, interest, rate);
        // let mut index = ACCOUNTS_INDEX.lock().unwrap();
        // index.insert(id, self.accounts.len());
        // self.accounts_index.insert(id, self.accounts.len());
        // self.accounts.push(account)
    // }
// 
    // pub fn transfer(
        // &'static mut self,
        // payment_id: &'static str,
        // collection_id: &'static str,
        // amount: f32,
    // ) {
        // let myself = Arc::new(Mutex::new(self));
        // let mut handles = vec![];
        // {
            // let myself = Arc::clone(&myself);
            // let handle =
                // spawn(
                    // move || match myself.lock().unwrap().accounts_index.get(payment_id) {
                        // None => None,
                        // Some(&index) => {
                            // Some(myself.lock().unwrap().accounts[index].withdraw(amount))
                        // }
                    // },
                // );
            // handles.push(handle);
        // }
        // {
            // let myself = Arc::clone(&myself);
            // let handle =
                // spawn(
                    // move || match myself.lock().unwrap().accounts_index.get(collection_id) {
                        // None => None,
                        // Some(&index) => {
                            // Some(myself.lock().unwrap().accounts[index].deposit(amount))
                        // }
                    // },
                // );
            // handles.push(handle);
        // }
        // for handle in handles {
            // handle.join().unwrap();
        // }
        // todo!();
    // }
// 
    // pub fn pay_out_wages(&'static mut self, id: &'static str, ammount: f32) {
        // spawn(move || match self.accounts_index.get(id) {
            // None => None,
            // Some(&index) => Some(self.accounts[index].deposit(ammount)),
        // });
        // /*
        // todo!();
        // */
    // }
// 
    // pub fn calculator_interest(&'static mut self, payment_id: &'static str) {
        // spawn(move || match self.accounts_index.get(&payment_id) {
            // None => None,
            // Some(&index) => Some(self.accounts[index].calculator_interest()),
        // });
        // /*
        // todo!();
        // */
    // }
// 
    // pub fn deposit(&'static mut self, id: &'static str, amount: f32) {
        // spawn(move || match self.accounts_index.get(&id) {
            // None => None,
            // Some(&index) => Some(self.accounts[index].deposit(amount)),
        // });
        // todo!()
    // }
// 
    // pub fn withdraw(&'static mut self, id: &'static str, amount: f32) {
        // spawn(move || match self.accounts_index.get(&id) {
            // None => None,
            // Some(&index) => Some(self.accounts[index].withdraw(amount)),
        // });
        // todo!()
    // }
// }
// 