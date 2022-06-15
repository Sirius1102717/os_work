pub use crate::account::Account;
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread::spawn;

static mut ACCOUNTS: Vec<Account> = Vec::new();
lazy_static! {
    static ref ACCOUNTS_INDEX: Mutex<HashMap<&'static str, usize>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

pub fn add_account(id: &'static str, blance: f32, interest: f32, rate: f32) {
    let account: Account = Account::new(id, blance, interest, rate);
    let mut index = ACCOUNTS_INDEX.lock().unwrap();
    unsafe{index.insert(id, ACCOUNTS.len())};
    unsafe { ACCOUNTS.push(account) }
}

pub fn transfer(
    payment_id: &'static str,
    collection_id: &'static str,
    amount: f32,
) {
    // let myself = Arc::new(Mutex::new(self));
    let mut handles = vec![];
    {
        // let myself = Arc::clone(&myself);
        let handle = spawn(
            move || match ACCOUNTS_INDEX.lock().unwrap().get(payment_id) {
                None => None,
                Some(&index) => unsafe{Some(ACCOUNTS[index].withdraw(amount))},
            },
        );
        handles.push(handle);
    }
    {
        let handle = spawn(
            move || match ACCOUNTS_INDEX.lock().unwrap().get(collection_id) {
                None => None,
                Some(&index) => unsafe{Some(ACCOUNTS[index].deposit(amount))},
            },
        );
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn pay_out_wages(id: &'static str, ammount: f32) {
    spawn(move || match ACCOUNTS_INDEX.lock().unwrap().get(id) {
        None => None,
        Some(&index) => unsafe{Some(ACCOUNTS[index].deposit(ammount))},
    });
}

pub fn calculator_interest(payment_id: &'static str) {
    spawn(move || match ACCOUNTS_INDEX.lock().unwrap().get(&payment_id) {
        None => None,
        Some(&index) => unsafe{Some(ACCOUNTS[index].calculator_interest())},
    });
}

pub fn deposit(id: &'static str, amount: f32) {
    spawn(move || match ACCOUNTS_INDEX.lock().unwrap().get(&id) {
        None => None,
        Some(&index) => unsafe{Some(ACCOUNTS[index].deposit(amount))},
    });
}

pub fn withdraw(id: &'static str, amount: f32) {
    spawn(move || match ACCOUNTS_INDEX.lock().unwrap().get(&id) {
        None => None,
        Some(&index) => unsafe{Some(ACCOUNTS[index].withdraw(amount))},
    });
}