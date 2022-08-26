#[macro_use]
extern crate lazy_static;
mod account;
mod bank;
mod bank_accounting_system;
mod btree;
mod byte;
mod page;
use bank_accounting_system as bas;
use btree::BTree;
use scanf::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::io::BufReader;
use std::rc::Rc;
use std::thread::spawn;
use std::time::Instant;

#[test]
fn build_index() {
    let mut map = BTree::<i32, f32>::new("./index.btree");
    let f = File::open("./account.csv").unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let mut id = 0i32;
        let mut amount: f32 = 0.0;
        sscanf!(&line.unwrap(), "{},{}", id, amount).unwrap();
        match map.get(&id) {
            Some(balance) => {
                map.set(&id, &(amount + balance)).unwrap();
            }
            None => {
                map.set(&id, &amount).unwrap();
            }
        }

        // map.insert(id.clone(), amount);
        // bank::add_account(id.clone());
        // bas::deposit(&id, amount);
    }
}

#[test]
fn run() {
    let transfer = |payment_id, collection_id, amount| {
        let mut map = BTree::<i32, f32>::new("./index.btree");
        
        let now = Instant::now();
        let blance = map.get(payment_id).unwrap();
        map.set(payment_id, &(blance - amount)).unwrap();
        let blance = map.get(collection_id).unwrap();
        map.set(payment_id, &(blance + amount)).unwrap();
        println!("转账所需时间:{}", now.elapsed().as_micros());
    };

    let user_is_exit = |id| {
        let mut map = BTree::<i32, f32>::new("./index.btree");
        let now = Instant::now();
        match map.get(id) {
            Some(_) => true,
            None => {
                println!("所需时间为：{}", now.elapsed().as_micros());
                eprintln!("{}该用户不存在!", id);
                // panic!("该用户不存在");
                false
            }
        }
    };

    let payment_id = 1179572757;
    let collection_id = 2045514596;
    let amount = 100f32;
    if user_is_exit(&payment_id) && user_is_exit(&collection_id) {
        transfer(&payment_id, &collection_id, amount);
    }

    let payment_id = 2064098435;
    let collection_id = 2119601595;
    let amount = 200f32;
    if user_is_exit(&payment_id) && user_is_exit(&collection_id) {
        transfer(&payment_id, &collection_id, amount);
    }

    let payment_id = 2011199898;
    let collection_id = 2085939837;
    let amount = 300f32;
    if user_is_exit(&payment_id) && user_is_exit(&collection_id) {
        transfer(&payment_id, &collection_id, amount);
    }
    let mut map = BTree::<i32, f32>::new("./index.btree");

    


}


fn main() {
    // bank::init_account();
    println!("银行记账系统 （20206848王潇）");
    println!("1、存款\n2、取款\n3、转账\n4、发工资\n5、发利息\n其他数字、退出");

    loop {
        // let mut line = String::new();
        // stdin().read_line(&mut line).unwrap();
        // let i: i32 = line.trim().parse().unwrap();
        let mut i = 6i32;
        scanf!("{}", i).unwrap();
        // unsafe {
        match i {
            1 => {
                println!("请输入存款人姓名和存款金额");
                let mut id = String::new();
                let mut amount = 0.0f32;
                scanf!("{}{}", id, amount).unwrap();
                bas::deposit(&id, amount);
            }
            2 => {
                println!("请输入取款人姓名和存款金额");
                let mut id = String::new();
                let mut amount = 0.0f32;
                scanf!("{}{}", id, amount).unwrap();
                bas::withdraw(&id, amount)
            }
            3 => {
                println!("请输入转账人姓名、收款人姓名和存款金额");
                let mut payment_id = String::new();
                let mut collection_id = String::new();
                let mut amount = 0.0f32;
                scanf!("{}{}{}", payment_id, collection_id, amount).unwrap();
                bas::transfer(&payment_id, &collection_id, amount);
            }
            4 => {
                println!("请输入发工资的组数");
                // let mut line = String::new();
                // stdin().read_line(&mut line).unwrap();
                // let n: usize = line.trim().parse().unwrap();
                let mut n = 0usize;
                scanf!("{}", n).unwrap();
                let mut operations = vec![];
                for _ in 0..n {
                    // INPUT = String::new();
                    let mut id = String::new();
                    let mut amount = 0.0f32;
                    println!("请输入需要发工资的账户名称和工资金额");
                    scanf!("{}{}", id, amount).unwrap();
                    // let amount: f32 = s.next().unwrap().parse().unwrap();
                    operations.push((id, amount));
                }
                bas::pay_out_wages(operations)
            }
            5 => {
                let mut n = 0usize;
                scanf!("{}", n).unwrap();
                let mut ids = vec![];
                for _ in 0..n {
                    // INPUT = String::new();
                    let mut id = String::new();
                    println!("请输入需要计算的账户名称");
                    scanf!("{}", id).unwrap();
                    // let amount: f32 = s.next().unwrap().parse().unwrap();
                    ids.push(id);
                }
                // bas::pay_out_wages(operations)
                // println!("请输入计算利息的组数");
                // let mut line = String::new();
                // stdin().read_line(&mut line).unwrap();
                // let n: usize = line.trim().parse().unwrap();
                // let mut ids = vec![];
                // for _ in 0..n {
                // println!("请输入需要计算的账户名称");
                // stdin().read_line(&mut INPUT).unwrap();
                // let id = &INPUT[..];
                // ids.push(id);
                // }
                bas::calculator_interest(ids)
            }
            _ => break,
        }
        // }
    }
}
