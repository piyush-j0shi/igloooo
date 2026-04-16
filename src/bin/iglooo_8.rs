use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn get_sequential_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug)]
enum Transaction {
    Deposite,
    Withdraw,
    Transfer,
}

#[derive(Debug)]
struct Transactionrecords {
    transactiontype: Transaction,
    amount: usize,
    timestamp: DateTime<Utc>,
}

#[derive(Debug)]
struct Account {
    uid: usize,
    name: String,
    balance: usize,
    transcation: Vec<Transactionrecords>,
}

impl Account {
    fn new_account(name: String, current_balance: usize) -> Self {
        let mut record: Vec<Transactionrecords> = Vec::new();
        if current_balance <= 0 {
            println!("starting balance can not be negative");

            let record_newaccount = Transactionrecords {
                transactiontype: Transaction::Deposite,
                amount: 0,
                timestamp: Utc::now(),
            };
            record.push(record_newaccount);
            Self {
                uid: get_sequential_id(),
                name: name,
                balance: 0,
                transcation: record,
            }
        } else {
            let record_newaccount = Transactionrecords {
                transactiontype: Transaction::Deposite,
                amount: current_balance,
                timestamp: Utc::now(),
            };
            record.push(record_newaccount);
            Self {
                uid: get_sequential_id(),
                name: name,
                balance: current_balance,
                transcation: record,
            }
        }
    }

    fn withdraw(&mut self, amount: usize) {
        if amount > self.balance {
            println!("not sufficient fund");
        } else if amount <= 0 {
            println!("amount can not be less than 0");
        } else {
            self.balance -= amount;

            let new_item = Transactionrecords {
                transactiontype: Transaction::Withdraw,
                amount: amount,
                timestamp: Utc::now(),
            };
            self.transcation.push(new_item);
        }
    }

    fn deposite(&mut self, amount: usize) {
        if amount <= 0 {
            println!("amount should be greater than 0");
        } else {
            self.balance += amount;

            let new_item = Transactionrecords {
                transactiontype: Transaction::Deposite,
                amount: amount,
                timestamp: Utc::now(),
            };
            self.transcation.push(new_item);
        }
    }

    fn get_details(&self) {
        println!("===================================================================");

        println!("name : {}", self.name);
        println!("current balance : {}", self.balance);

        println!("transaction history");
        for item in &self.transcation {
            println!("item : {:?}", item);
        }
        println!("===================================================================");
    }

    fn self_transfer(&mut self, amount: usize, other_account: &mut Account) {
        if amount <= 0 {
            println!("value can not be negative");
        } else if amount > self.balance {
            println!("not sufficient fund");
        } else {
            self.balance -= amount;
            other_account.balance += amount;

            let self_account = Transactionrecords {
                transactiontype: Transaction::Transfer,
                amount: amount,
                timestamp: Utc::now(),
            };

            let other_account_state = Transactionrecords {
                transactiontype: Transaction::Transfer,
                amount: amount,
                timestamp: Utc::now(),
            };

            self.transcation.push(self_account);
            other_account.transcation.push(other_account_state);
        }
    }

    fn accont_transfer(account_1: &mut Account, amount: usize, account_2: &mut Account) {
        if amount <= 0 {
            println!("value needs to be grater than 0");
        } else if amount > account_1.balance {
            println!("insufficient f0und");
        } else {
            account_1.balance -= amount;
            account_2.balance += amount;

            let account_1_record = Transactionrecords {
                transactiontype: Transaction::Transfer,
                amount: amount,
                timestamp: Utc::now(),
            };

            let account_2_record = Transactionrecords {
                transactiontype: Transaction::Transfer,
                amount: amount,
                timestamp: Utc::now(),
            };

            account_1.transcation.push(account_1_record);
            account_2.transcation.push(account_2_record);
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input.trim().to_string()
}

fn execute() {
    println!("enter name : ");
    let user_name = read_input();

    println!("enter inital balance : ");
    let user_balance = read_input().parse().expect("failed to read a number");

    let mut user = Account::new_account(user_name, user_balance);
    user.get_details();

    println!("other account");
    let mut user_1 = Account::new_account(String::from("user2"), user_balance);
    user_1.get_details();

    loop {
        println!("=================================================================================================================================================================");
        println!("select an operation : get : getdetails | draw :withdraw  | site : deposite | exit : exit loop | self : self transferr | transfer : Transfer one to another account");
        let operation_input = read_input().trim().to_lowercase();

        if operation_input == "draw" {
            println!("enter withdraw amount");
            let draw_amount: usize = read_input().parse().expect("failed to read a number");

            user.withdraw(draw_amount);
            user.get_details();
        } else if operation_input == "site" {
            println!("enter withdraw amount");
            let site_amount: usize = read_input().parse().expect("failed to read a number");

            user.deposite(site_amount);
            user.get_details();
        } else if operation_input == "get" {
            user.get_details();
        } else if operation_input == "exit" {
            break;
        } else if operation_input == "self" {
            user.self_transfer(100, &mut user_1);
            user.get_details();
        } else if operation_input == "transfer" {
            println!("Enter transfer amount:");
            let transfer_amount: usize = read_input().parse().expect("failed to read number");
            Account::accont_transfer(&mut user, transfer_amount, &mut user_1);
            user.get_details();
            user_1.get_details();
        }
    }

    user_1.get_details();
}

fn main() {
    execute();
}

// I tried to write this code for multiple account and account retreval also using
// Hashmap<usize, Account> but I was going then I stopped, will see some other time.

// fn multiple_accounts() {
//     let account_manager: HashMap<usize, Account> = HashMap::new();

//     println!("this is account manager");
//     println!("do you want a new account or get old account : new | get ");
//     println!("enter you choice");

//     let choice = read_input().trim().to_lowercase();
//     if choice == "ret" {
//         println!("enter your account id");
//         let account_id: usize = read_input().parse().expect("failed to read");

//         if let Some(user) = account_manager.get(&account_id) {
//             user.get_details();
//         } else {
//             println!("no any account found");
//         }
//     } else if choice == "new" {
//         println!("enter name : ");
//         let user_name = read_input();

//         println!("enter inital balance : ");
//         let user_balance = read_input().parse().expect("failed to read a number");

//         let mut user = Account::new_account(user_name, user_balance);
//         execute(&mut user);
//     } else {
//         println!("invaliud choice");
//     }
// }

// fn execute(user: &mut Account) {
//     // println!("enter name : ");
//     // let user_name = read_input();

//     // println!("enter inital balance : ");
//     // let user_balance = read_input().parse().expect("failed to read a number");

//     // let mut user = Account::new_account(user_name, user_balance);
//     // user.get_details();

//     // println!("other account");
//     // let mut user_1 = Account::new_account(String::from("user2"), user_balance);
//     // user_1.get_details();

//     loop {
//         println!("===========================================================================================================================================================================================");
//         println!("select an operation : get : getdetails | draw :withdraw  | site : deposite | exit : exit loop | self : self transferr | transfer : Transfer one to another account | ret : retrieve account");
//         let operation_input = read_input().trim().to_lowercase();

//         if operation_input == "draw" {
//             println!("enter withdraw amount");
//             let draw_amount: usize = read_input().parse().expect("failed to read a number");

//             user.withdraw(draw_amount);
//             user.get_details();
//         } else if operation_input == "site" {
//             println!("enter withdraw amount");
//             let site_amount: usize = read_input().parse().expect("failed to read a number");

//             user.deposite(site_amount);
//             user.get_details();
//         } else if operation_input == "get" {
//             user.get_details();
//         } else if operation_input == "exit" {
//             break;
//         } else if operation_input == "self" {
//             // user.self_transfer(100, &mut user_1);
//             user.get_details();
//         } else if operation_input == "transfer" {
//             println!("Enter transfer amount:");
//             // let transfer_amount: usize = read_input().parse().expect("failed to read number");
//             // Account::accont_transfer(&mut user, transfer_amount, &mut user_1);
//             // user.get_details();
//             // user_1.get_details();
//         } else if operation_input == "ret" {
//             multiple_accounts();
//         }
//     }

//     // user_1.get_details();
// }

// fn main() {
//     execute();
// }
