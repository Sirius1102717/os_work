use os_work::bank;
use os_work::bank_accounting_system as bas;
use std::io::stdin;

pub mod bbank;
static mut INPUT: String = String::new();

fn main() {
    bank::init_account();

    println!("银行记账系统 （20206848王潇）");
    println!("1、存款\n2、取款\n3、转账\n4、发工资\n5、发利息\n其他数字、退出");

    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let i: i32 = line.trim().parse().unwrap();

        unsafe {
            match i {
                1 => {
                    println!("请输入存款人姓名");
                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap();
                    // let id: &str = input.as_str();
                    INPUT = input.clone();
                    println!("请输入存款金额");
                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap();
                    let amount: f32 = input.trim().parse().unwrap();
                    bas::deposit(INPUT.as_str(), amount)
                }
                2 => {
                    println!("请输入取款人姓名和取款款金额并以空格分割");
                    stdin().read_line(&mut INPUT).unwrap();
                    let mut s = INPUT.split_whitespace();
                    let id: &str = s.next().unwrap();
                    let amount: f32 = s.next().unwrap().parse().unwrap();
                    bas::withdraw(id, amount)
                }
                3 => {
                    println!("请输入转帐人姓名、收款人姓名以及转账金额并以空格分割");
                    stdin().read_line(&mut INPUT).unwrap();
                    let mut s = INPUT.split_whitespace();
                    let payment_id: &str = s.next().unwrap();
                    let collection_id: &str = s.next().unwrap();
                    let amount: f32 = s.next().unwrap().parse().unwrap();
                    bas::transfer(payment_id, collection_id, amount)
                }
                4 => {
                    println!("请输入发工资的组数");
                    let mut line = String::new();
                    stdin().read_line(&mut line).unwrap();
                    let n: usize = line.trim().parse().unwrap();
                    let mut operations = vec![];
                    for _ in 0..n {
                        INPUT = String::new();
                        println!("请输入需要发工资的账户名称和工资金额并以空格分割");
                        stdin().read_line(&mut INPUT).unwrap();
                        let mut s = INPUT.split_whitespace();
                        let id: &str = s.next().unwrap();
                        let tmp = s.next().unwrap();
                        let amount: f32 = tmp.parse().unwrap();
                        // let amount: f32 = s.next().unwrap().parse().unwrap();
                        operations.push((id, amount));
                    }
                    bas::pay_out_wages(operations)
                }
                5 => {
                    println!("请输入计算利息的组数");
                    let mut line = String::new();
                    stdin().read_line(&mut line).unwrap();
                    let n: usize = line.trim().parse().unwrap();
                    let mut ids = vec![];
                    for _ in 0..n {
                        println!("请输入需要计算的账户名称");
                        stdin().read_line(&mut INPUT).unwrap();
                        let id = &INPUT[..];
                        ids.push(id);
                    }
                    bas::calculator_interest(ids)
                }
                _ => break,
            }
        }
    }
}
