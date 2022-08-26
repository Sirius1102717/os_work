extern crate lazy_static;
use crate::bank;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;

// static mut TRANSFER_LIST: Vec<(&str, &str, f32)> = Vec::new();
// static mut PAY_OUT_WAGES_LIST: Vec<(&str, f32)> = Vec::new();
// static mut CALCULATOR_INTEREST_LIST: Vec<&str> = Vec::new();
// static mut DEPOSIT_LIST: Vec<(&str, f32)> = Vec::new();
// static mut WITHDRAW_LIST: Vec<(&str, f32)> = Vec::new();

enum _Operation {
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

lazy_static! {
    // static ref THREAD_PRIORITY: Mutex<bool> = Mutex::new(true);
    // static ref THREAD_PRIORITY: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    // static ref CVAD: Arc<Condvar> = Arc::new(Condvar::new());

    static ref PAIR: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(true), Condvar::new()));
    // static ref COND: Condvar = Condvar::new();
}
//
struct _Handle {
    pub priority: bool,
    pub operation: _Operation,
}

impl _Handle {
    const fn _new(priority: bool, operation: _Operation) -> Self {
        _Handle {
            priority,
            operation,
        }
    }
}
//
pub fn transfer(payment_id: &str, collection_id: &str, amount: f32) {
    // let handle = Handle::new(
    // true,
    // Operation::Transfer {
    // payment_id,
    // collection_id,
    // amount,
    // },
    // );
    // add(handle);
    // let mut priority = THREAD_PRIORITY.try_lock().unwrap();
    // sleep(Duration::from_micros(2000_000));
    // let mut priority = THREAD_PRIORITY.try_lock().expect("tran_pri_err");
    // *priority = false;
    // *priority = true;
    let (lock, cvar) = &*Arc::clone(&PAIR);
    let mut started = lock.lock().unwrap();
    *started = false;
    bank::transfer(payment_id, collection_id, amount);
    *started = true;
    cvar.notify_all();
    // sleep(Duration::from_micros(2000_000));
}
pub fn deposit(id: &str, amount: f32) {
    // let mut priority = THREAD_PRIORITY.try_lock().unwrap();
    // {
    // let mut priority = THREAD_PRIORITY.try_lock().expect("depo_pri_err");
    // *priority = false;
    // bank::deposit(id, amount);
    // *priority = true;
    // }
    // let cvad = Arc::clone(&CVAD);
    // cvad.notify_all();
    // let handle = Handle::new(true, Operation::Deposit { id, amount });
    // add(handle);
    let (lock, cvar) = &*Arc::clone(&PAIR);
    let mut started = lock.lock().unwrap();
    *started = false;
    bank::deposit(id, amount);
    *started = true;
    cvar.notify_all();
}

pub fn withdraw(id: &str, amount: f32) {
    // let handle = Handle::new(true, Operation::Withdraw { id, amount });
    // add(handle);
    // let mut priority = THREAD_PRIORITY.try_lock().unwrap();
    // {
    // let mut priority = THREAD_PRIORITY.try_lock().expect("with_pri_err");
    // *priority = false;
    // bank::withdraw(id, amount);
    // *priority = true;
    // }
    // let cvad = Arc::clone(&CVAD);
    // cvad.notify_all();
    //
    let (lock, cvar) = &*Arc::clone(&PAIR);
    let mut started = lock.lock().unwrap();
    *started = false;
    bank::withdraw(id, amount);
    *started = true;
    cvar.notify_all();
}

pub fn pay_out_wages(operations: Vec<(String, f32)>) {
    // let handle = Handle::new(false, Operation::PayOutWages(operations));
    // add(handle);

    // let mut priority = THREAD_PRIORITY.try_lock().unwrap();
    // let mut priority = THREAD_PRIORITY.try_lock().expect("pay_pri_err");
    // let mut priority = THREAD_PRIORITY.lock().expect("pay_pri_err");
    // *priority = false;
    // let cvad = Arc::clone(&CVAD);
    // started = false;
    let mut handles = vec![];
    // let len = operations.len();
    for operation in operations {
        // while !*priority {
        // priority = cvad.wait(priority).unwrap();
        // priority = cvad.wait(priority).expect("pay_wait_err");
        // }
        // sleep(Duration::from_micros(1000_00));
        let handle = spawn(move || {
            let (lock, cvar) = &*Arc::clone(&PAIR);
            let mut started = lock.lock().unwrap();
            while !*started {
                started = cvar.wait(started).unwrap();
            }
            bank::pay_out_wages(&operation.0, operation.1);
        });
        // handle.join().unwrap();
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    sleep(Duration::from_micros(1000_00));
}

pub fn calculator_interest(ids: Vec<String>) {
    // let handle = Handle::new(false, Operation::CalculatorInterest(ids));
    // add(handle);
    // let mut priority = THREAD_PRIORITY.try_lock().expect("cal_pri_err");
    // *priority = false;
    let mut handles = vec![];
    // let cvad = Arc::clone(&CVAD);
    for id in ids {
        // while !*priority {
        // priority = cvad.wait(priority).expect("cal_wait_err");
        // }
        // sleep(Duration::from_micros(1000_00));
        let handle = spawn(move || {
            let (lock, cvar) = &*Arc::clone(&PAIR);
            let mut started = lock.lock().unwrap();
            while !*started {
                started = cvar.wait(started).unwrap();
            }
            bank::calculator_interest(&id);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    sleep(Duration::from_micros(1000_00));
    // *priority = true;
}

#[cfg(test)]
mod test {

    use std::thread::sleep;

    use super::*;

    #[test]
    fn all_test() {
        bank::init_account();

        let handle = spawn(|| {
            spawn(|| {
                let ids = vec![String::from("Ava"), String::from("Bella"), String::from("Carol"), String::from("Diana"), String::from("Eileen")];
                calculator_interest(ids);
                println!("After calculator_interest for 5 people");
                let ids = vec!["Ava", "Bella", "Carol", "Diana", "Eileen"];
                for id in ids {
                    println!("{:#?}", bank::show_account(id));
                }
            });
            spawn(|| {
                let operations = vec![
                    (String::from("Ava"), 300.0),
                    (String::from("Bella"), 300.0),
                    (String::from("Carol"), 300.0),
                    (String::from("Diana"), 300.0),
                    (String::from("Eileen"), 300.0),
                ];

                pay_out_wages(operations);
                println!("After pay_out_wages for 5 people");
                let ids = vec!["Ava", "Bella", "Carol", "Diana", "Eileen"];
                for id in ids {
                    println!("{:#?}", bank::show_account(id));
                }
            });
            spawn(|| {
                deposit("Ava", 300.0);
                let account = bank::show_account("Ava");
                println!("After deposit 300.0 to Ava: {:#?}", account);
            });
            //
            spawn(|| {
                withdraw("Diana", 300.0);
                let account = bank::show_account("Diana");
                println!("After withdraw 300.0 from Diana: {:#?}", account);
            });
            spawn(|| {
                transfer("Carol", "Bella", 300.0);
                let account = bank::show_account("Carol");
                println!("After transfter 300.0 from Carol to Bella ");
                println!("{:#?}", account);
                let account = bank::show_account("Bella");
                println!("{:#?}", account);
            });
        });
        handle.join().unwrap();
        sleep(Duration::from_secs(5));
    }
}
