pub use crate::account::Account;
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread::{spawn, sleep};
use std::time::Duration;

static mut ACCOUNTS: Vec<Account> = Vec::new();
lazy_static! {
    static ref ACCOUNTS_MAP: Mutex<HashMap<&'static str, usize>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

pub fn add_account(id: &'static str, blance: f32, interest: f32, rate: f32) {
    let account: Account = Account::new(id, blance, interest, rate);
    let mut index = ACCOUNTS_MAP.lock().unwrap();
    unsafe { index.insert(id, ACCOUNTS.len()) };
    unsafe { ACCOUNTS.push(account) }
}

pub fn show_account(id: &'static str) -> &Account {
    let map = ACCOUNTS_MAP.lock().unwrap();
    let index = map.get(id).unwrap();

    unsafe { ACCOUNTS.get(*index).unwrap() }
}

pub fn transfer(payment_id: &'static str, collection_id: &'static str, amount: f32) {
    // let myself = Arc::new(Mutex::new(self));
    let mut handles = vec![];
    {
        // let myself = Arc::clone(&myself);
        let handle = spawn(move || match ACCOUNTS_MAP.lock().unwrap().get(payment_id) {
            None => None,
            Some(&index) => unsafe { Some(ACCOUNTS[index].withdraw(amount)) },
        });
        handles.push(handle);
    }
    {
        let handle = spawn(
            move || match ACCOUNTS_MAP.lock().unwrap().get(collection_id) {
                None => None,
                Some(&index) => unsafe { Some(ACCOUNTS[index].deposit(amount)) },
            },
        );
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn pay_out_wages(id: &'static str, ammount: f32) {
    let &index = ACCOUNTS_MAP.lock().unwrap().get(id).unwrap();
    unsafe {
        ACCOUNTS[index].deposit(ammount);
    }
    // match ACCOUNTS_MAP.lock().unwrap().get(id) {
        // None => None,
        // Some(&index) => unsafe { Some(ACCOUNTS[index].deposit(ammount)) },
    // });
    // handle.join().unwrap();
}

pub fn calculator_interest(id: &'static str) {
    let &index = ACCOUNTS_MAP.lock().unwrap().get(id).unwrap();
    unsafe {
        ACCOUNTS[index].calculator_interest();
    }
    // match ACCOUNTS_MAP.lock().unwrap().get(payment_id) {
    // None => None,
    // Some(&index) => unsafe { Some(ACCOUNTS[index].calculator_interest()) },
    // }
}

pub fn deposit(id: &'static str, amount: f32) {
    let &index = ACCOUNTS_MAP.lock().unwrap().get(id).unwrap();
    unsafe {
        ACCOUNTS[index].deposit(amount);
    }

    // let handle = spawn(move || match ACCOUNTS_MAP.lock().unwrap().get(id) {
    // None => None,
    // Some(&index) => unsafe { Some(ACCOUNTS[index].deposit(amount)) },
    // });
    // handle.join().unwrap();
}

pub fn withdraw(id: &'static str, amount: f32) {
    let &index = ACCOUNTS_MAP.lock().unwrap().get(id).unwrap();
    unsafe {
        ACCOUNTS[index].withdraw(amount);
    }
    // let handle = spawn(move || match ACCOUNTS_MAP.lock().unwrap().get(id) {
    // None => None,
    // Some(&index) => unsafe { Some(ACCOUNTS[index].withdraw(amount)) },
    // });
    // handle.join().unwrap();
}
pub fn init_account() {
    add_account("Ava", 1000.0, 100.0, 0.1);
    add_account("Bella", 1000.0, 100.0, 0.1);
    add_account("Carol", 1000.0, 100.0, 0.1);
    add_account("Diana", 1000.0, 100.0, 0.1);
    add_account("Eileen", 1000.0, 100.0, 0.1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transfer_test() {
        unsafe {
            ACCOUNTS.clear();
        }
        init_account();
        transfer("Carol", "Bella", 300.0);
        let acoount_carol = show_account("Carol");
        let acoount_bella = show_account("Bella");
        assert_eq!(700.0, acoount_carol.blance());
        assert_eq!(1300.0, acoount_bella.blance());
    }

    #[test]
    fn deposit_test() {
        unsafe {
            ACCOUNTS.clear();
        }
        init_account();
        deposit("Ava", 300.0);
        let account = show_account("Ava");
        assert_eq!(1300.0, account.blance());
    }

    #[test]
    fn withdraw_test() {
        unsafe {
            ACCOUNTS.clear();
        }
        init_account();
        withdraw("Diana", 300.0);
        let account = show_account("Diana");
        assert_eq!(700.0, account.blance());
    }

    #[test]
    fn pay_out_wages_test() {
        // init_account();
        unsafe {
            ACCOUNTS.clear();
            init_account();
            for account in &ACCOUNTS {
                pay_out_wages(account.id(), 300.0);
            }
            for account in &ACCOUNTS {
                // pay_out_wages(account.id(), 300.0);
                assert_eq!(1300.0, account.blance());
            }
        }
    }

    #[test]
    fn calculator_interest_test() {
        // init_account();
        unsafe {
            ACCOUNTS.clear();
            init_account();
            for account in &ACCOUNTS {
                calculator_interest(account.id());
            }
            for account in &ACCOUNTS {
                // pay_out_wages(account.id(), 300.0);
                assert_eq!(200.0, account.interest());
            }
        }
    }
}
