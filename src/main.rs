#[macro_use]
extern crate lazy_static;
use std::io::stdin;
pub mod account;
pub mod bank;
pub mod bank_accounting_system;
pub mod bbank;
static mut INPUT: String = String::new();
fn main() {
    println!("银行记账系统 （20206848王潇）");
    println!("1、存款\n2、取款\n3、转账\n4、发工资\n5、发利息\n其他数字、退出");
    let mut line = String::new();

    loop {
        let i: i32 = stdin().read_line(&mut line).unwrap().try_into().unwrap();
        unsafe {
            match i {
                1 => {
                    stdin().read_line(&mut INPUT).unwrap();
                    let mut s = INPUT.split_whitespace();
                    let id: &str = s.next().unwrap();
                    let amount: f32 = s.next().unwrap().parse().unwrap();
                    bank_accounting_system::deposit(id, amount)
                }
                2 => {
                    stdin().read_line(&mut INPUT).unwrap();
                    let mut s = INPUT.split_whitespace();
                    let id: &str = s.next().unwrap();
                    let amount: f32 = s.next().unwrap().parse().unwrap();
                    bank_accounting_system::withdraw(id, amount)
                }
                3 => {
                    stdin().read_line(&mut INPUT).unwrap();
                    let mut s = INPUT.split_whitespace();
                    let payment_id: &str = s.next().unwrap();
                    let collection_id: &str = s.next().unwrap();
                    let amount: f32 = s.next().unwrap().parse().unwrap();
                    bank_accounting_system::transfer(payment_id, collection_id, amount)
                }
                4 => {
                    println!("请输入发工资的组数");
                    let n: usize = stdin().read_line(&mut INPUT).unwrap().try_into().unwrap();
                    let mut operations = vec![];
                    for _ in 0..n {
                        stdin().read_line(&mut INPUT).unwrap();
                        let mut s = INPUT.split_whitespace();
                        let id: &str = s.next().unwrap();
                        let amount: f32 = s.next().unwrap().parse().unwrap();
                        operations.push((id, amount));
                    }
                    bank_accounting_system::pay_out_wages(operations)
                }
                5 => {
                    println!("请输入计算利息的组数");
                    let n: usize = stdin().read_line(&mut INPUT).unwrap().try_into().unwrap();
                    let mut ids = vec![];
                    for _ in 0..n {
                        stdin().read_line(&mut INPUT).unwrap();
                        let id = &INPUT[..];
                        ids.push(id);
                    }
                    bank_accounting_system::calculator_interest(ids)
                }
                _ => break,
            }
        }
    }
}
