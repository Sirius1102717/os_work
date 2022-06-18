extern crate lazy_static;
pub use crate::account::Account;
pub use crate::bank;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::spawn;
use std::time::Duration;

// static mut TRANSFER_LIST: Vec<(&str, &str, f32)> = Vec::new();
// static mut PAY_OUT_WAGES_LIST: Vec<(&str, f32)> = Vec::new();
// static mut CALCULATOR_INTEREST_LIST: Vec<&str> = Vec::new();
// static mut DEPOSIT_LIST: Vec<(&str, f32)> = Vec::new();
// static mut WITHDRAW_LIST: Vec<(&str, f32)> = Vec::new();

enum Operation {
    Transfer {
        payment_id: &'static str,
        collection_id: &'static str,
        amount: f32,
    },
    PayOutWages(Vec<(&'static str, f32)>),
    CalculatorInterest(Vec<&'static str>),
    Deposit {
        id: &'static str,
        amount: f32,
    },
    Withdraw {
        id: &'static str,
        amount: f32,
    },
}

static mut THREAD_PRIORITY: AtomicBool = AtomicBool::new(false);

struct Handle {
    pub priority: bool,
    pub operation: Operation,
}

impl Handle {
    const fn new(priority: bool, operation: Operation) -> Self {
        Handle {
            priority,
            operation,
        }
    }
}

pub fn transfer(payment_id: &'static str, collection_id: &'static str, amount: f32) {
    let handle = Handle::new(
        true,
        Operation::Transfer {
            payment_id,
            collection_id,
            amount,
        },
    );
    add(handle);
}
pub fn deposit(id: &'static str, amount: f32) {
    let handle = Handle::new(true, Operation::Deposit { id, amount });
    add(handle);
}

pub fn withdraw(id: &'static str, amount: f32) {
    let handle = Handle::new(true, Operation::Withdraw { id, amount });
    add(handle);
}

pub fn pay_out_wages(operations: Vec<(&'static str, f32)>) {
    let handle = Handle::new(false, Operation::PayOutWages(operations));
    add(handle);
}

pub fn calculator_interest(ids: Vec<&'static str>) {
    let handle = Handle::new(false, Operation::CalculatorInterest(ids));
    add(handle);
}

fn add(oper: Handle) {
    unsafe {
        if oper.priority {
            // let priotity = THREAD_PRIORITY.get_mut();
            THREAD_PRIORITY.store(true, Ordering::SeqCst);
            // *priotity = true;
            match oper.operation {
                Operation::Transfer {
                    payment_id,
                    collection_id,
                    amount,
                } => {
                    bank::transfer(payment_id, collection_id, amount);
                    THREAD_PRIORITY.store(false, Ordering::SeqCst);
                    // let priotity = THREAD_PRIORITY.get_mut();
                    // *priotity = false;
                }
                Operation::Deposit { id, amount } => {
                    bank::deposit(id, amount);
                    THREAD_PRIORITY.store(false, Ordering::SeqCst);
                    // let priotity = THREAD_PRIORITY.get_mut();
                    // *priotity = false;
                }
                Operation::Withdraw { id, amount } => {
                    bank::withdraw(id, amount);
                    THREAD_PRIORITY.store(false, Ordering::SeqCst);
                    // let priotity = THREAD_PRIORITY.get_mut();
                    // *priotity = false;
                }
                _ => (),
            }
        } else {
            // let priotity = THREAD_PRIORITY.get_mut();
            while THREAD_PRIORITY.load(Ordering::SeqCst) {
                std::thread::sleep(Duration::from_millis(100));
            }
            match oper.operation {
                Operation::CalculatorInterest(ids) => {
                    for id in ids {
                        while THREAD_PRIORITY.load(Ordering::SeqCst) {
                            std::thread::sleep(Duration::from_millis(100));
                        }
                        let handle = spawn(move || {
                            bank::calculator_interest(id);
                            std::thread::sleep(Duration::from_millis(500));
                        });
                        handle.join().unwrap();
                    }
                }
                Operation::PayOutWages(operations) => {
                    for operation in operations {
                        while THREAD_PRIORITY.load(Ordering::SeqCst) {
                            std::thread::sleep(Duration::from_millis(100));
                        }
                        let handle = spawn(move || {
                            bank::pay_out_wages(operation.0, operation.1);
                        });
                        handle.join().unwrap();
                    }
                }
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn all_test() {
        bank::init_account();

        let handle = spawn(|| {
            // spawn(|| {
            // transfer("Carol", "Bella", 300.0);
            // let account = bank::show_account("Carol");
            // println!("After transfter 300.0 from Carol to Bella ");
            // println!("{:#?}", account);
            // assert_eq!(700.0, account.blance());
            // let account = bank::show_account("Bella");
            // assert_eq!(1300.0, account.blance());
            // println!("{:#?}", account);
            // });
            // spawn(|| {
            // deposit("Ava", 300.0);
            // let account = bank::show_account("Ava");
            // println!("After deposit 300.0 to Ava: {:#?}", account);
            // assert_eq!(1300.0, account.blance());
            // });
            //
            // spawn(|| {
            // withdraw("Diana", 300.0);
            // let account = bank::show_account("Diana");
            // println!("After withdraw 300.0 from Diana: {:#?}", account);
            // assert_eq!(700.0, account.blance());
            // });

            spawn(|| {
                let ids = vec!["Ava", "Bella", "Carol", "Diana", "Eileen"];
                calculator_interest(ids);
                println!("After calculator_interest for 5 people");
                let ids = vec!["Ava", "Bella", "Carol", "Diana", "Eileen"];
                for id in ids {
                    println!("{:#?}", bank::show_account(id));
                }
            });

            // spawn(|| {
            // let operations = vec![
            // ("Ava", 300.0),
            // ("Bella", 300.0),
            // ("Carol", 300.0),
            // ("Diana", 300.0),
            // ("Eileen", 300.0),
            // ];
            // pay_out_wages(operations);
            // println!("After pay_out_wages for 5 people");
            // let ids = vec!["Ava", "Bella", "Carol", "Diana", "Eileen"];
            // for id in ids {
            // println!("{:#?}", bank::show_account(id));
            // }
            // });
        });
        handle.join().unwrap();
    }
}
