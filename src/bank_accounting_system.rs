extern crate lazy_static;
pub use crate::account::Account;
pub use crate::bank;
use std::sync::atomic::AtomicBool;
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
    spawn(move || add(handle));
}
pub fn deposit(id: &'static str, amount: f32) {
    let handle = Handle::new(true, Operation::Deposit { id, amount });
    spawn(move || add(handle));
}

pub fn withdraw(id: &'static str, amount: f32) {
    let handle = Handle::new(true, Operation::Withdraw { id, amount });
    spawn(move || add(handle));
}

pub fn pay_out_wages(operations: Vec<(&'static str, f32)>) {
    let handle = Handle::new(false, Operation::PayOutWages(operations));
    spawn(move || add(handle));
}

pub fn calculator_interest(ids: Vec<&'static str>) {
    let handle = Handle::new(false, Operation::CalculatorInterest(ids));
    spawn(move || add(handle));
}

fn add(oper: Handle) {
    unsafe {
        if oper.priority {
            let priotity = THREAD_PRIORITY.get_mut();
            *priotity = true;
            match oper.operation {
                Operation::Transfer {
                    payment_id,
                    collection_id,
                    amount,
                } => {
                    let handle = spawn(move || {
                        bank::transfer(payment_id, collection_id, amount);
                        let priotity = THREAD_PRIORITY.get_mut();
                        *priotity = false;
                    });
                    handle.join().unwrap();
                }
                Operation::Deposit { id, amount } => {
                    let handle = spawn(move || {
                        bank::deposit(id, amount);
                        let priotity = THREAD_PRIORITY.get_mut();
                        *priotity = false;
                    });
                    handle.join().unwrap();
                }
                Operation::Withdraw { id, amount } => {
                    let handle = spawn(move || {
                        bank::withdraw(id, amount);
                        let priotity = THREAD_PRIORITY.get_mut();
                        *priotity = false;
                    });
                    handle.join().unwrap();
                }
                _ => (),
            }
        } else {
            let priotity = THREAD_PRIORITY.get_mut();
            if !*priotity {
                match oper.operation {
                    Operation::CalculatorInterest(ids) => {
                        for id in ids {
                            while *priotity {
                                std::thread::sleep(Duration::from_millis(100));
                            }
                            let handle = spawn(move || {
                                bank::calculator_interest(id);
                            });
                            handle.join().unwrap();
                        }
                    }
                    Operation::PayOutWages(operations) => {
                        for operation in operations {
                            while *priotity {
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
}
