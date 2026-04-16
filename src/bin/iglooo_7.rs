use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};

// need to import `Write` here to use create_file() function
use std::io::{self, BufReader, BufWriter};

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Bought,
    NotBought,
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    name: String,
    status: Status,
}

fn read_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");
    user_input.trim().to_string()
}

// fn create_file() {
//     let file = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("data.json")
//         .expect("failed to open file");
//     let mut writer = BufWriter::new(file);
//     writeln!(writer, "{{\"message\": \"Hello\"}}").unwrap();
// }

fn add_item(null_items: &mut Vec<Item>) {
    println!("enter the product you want to add");
    let product_name = read_input().trim().to_string();
    let lowercase_productname = product_name.to_lowercase();

    if null_items
        .iter()
        .any(|s| s.name.to_lowercase() == lowercase_productname)
    {
        println!("item already exists");
        // return null_items;
    } else {
        let items = Item {
            name: product_name,
            status: Status::NotBought,
        };

        null_items.push(items);

        // let file = OpenOptions::new()
        //     .write(true)
        //     .truncate(true)
        //     .create(true)
        //     .open("data.json")
        //     .expect("failed to open file");
        // let writer = BufWriter::new(file);

        // serde_json::to_writer_pretty(writer, null_items).expect("failed to write json");
        // println!("item added");

        save_data(null_items);
        println!("item added");

        // null_items
    }
}

fn remove_item(items_list: &mut Vec<Item>) {
    if items_list.is_empty() {
        println!("there are not items")
    } else {
        view_item(items_list);
        println!("enter the item name you want to remove");
        let removed_item = read_input();

        if let Some(index) = items_list
            .iter()
            .position(|index| index.name.to_lowercase() == removed_item)
        {
            items_list.remove(index);
            save_data(&items_list);
            println!("item removed");
        } else {
            println!("no intem found");
        }
    }
}

fn view_item(some_items: &Vec<Item>) {
    if some_items.is_empty() {
        println!("there are no items, add some items to view items")
    }

    for (index, item) in some_items.iter().enumerate() {
        println!(
            "item no : {} | item name : {:?} | item status : {:?}",
            index, item.name, item.status
        );
    }
}

fn buy_items(added_items: &mut Vec<Item>) {
    if added_items.is_empty() {
        println!("there are no items at this moment");
    } else {
        view_item(added_items);
        println!("enter the item name you want to buy");
        let buying_item = read_input().trim().to_lowercase();

        if let Some(item) = added_items
            .iter_mut()
            .find(|item| item.name.to_lowercase() == buying_item)
        {
            item.status = Status::Bought;
            save_data(&added_items);
            println!("item marked as bought");
        } else {
            println!("item no found");
        }
    }
}

fn unbought_items(bought_items: &mut Vec<Item>) {
    if bought_items.is_empty() {
        println!("there are not any items");
    } else {
        view_item(bought_items);
        println!("enter the item name you want to buy");
        let unbuying_item = read_input().trim().to_lowercase();

        if let Some(item) = bought_items
            .iter_mut()
            .find(|item| item.name.to_lowercase() == unbuying_item)
        {
            match item.status {
                Status::NotBought => println!("already marked as bought"),
                Status::Bought => {
                    item.status = Status::NotBought;
                    save_data(bought_items);
                    println!("marked as unbuy");
                }
            }
        } else {
            println!("item no found");
        }
    }
}

fn load_data() -> Vec<Item> {
    let file = File::open("data.json").expect("failed to open json");
    let reader = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(data) => {
            let some_itemslist: Vec<Item> = data;
            return some_itemslist;
        }

        #[allow(unused_variables)]
        Err(e) => {
            let some_itemslist: Vec<Item> = Vec::new();
            return some_itemslist;
        }
    }
    // some_itemslist
}

fn save_data(items_list: &Vec<Item>) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("data.json")
        .expect("failed to open file");
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, items_list).expect("failed to write json");
}

fn execute(some_itemslist: &mut Vec<Item>) {
    loop {
        println!("\n");
        println!("============================================");
        println!("add | buy | unbuy |view | remove | type exit");
        println!("============================================");

        // println!("type anything");
        // let input = read_input();
        // println!("you typed : {}", input);

        // let first_item = add_item(&mut shopping_list);
        // println!("item is : {:?}", first_item);

        let new_input = read_input().trim().to_lowercase();
        if new_input == "add" {
            add_item(some_itemslist);
        } else if new_input == "buy" {
            buy_items(some_itemslist);
        } else if new_input == "view" {
            view_item(some_itemslist);
        } else if new_input == "remove" {
            remove_item(some_itemslist);
        } else if new_input == "unbuy" {
            unbought_items(some_itemslist);
        } else if new_input == "exit" {
            break;
        } else {
            println!("invalid input");
        }
    }
}

// todo : fix reading empty file
fn main() {
    // let mut shopping_list: Vec<Item> = Vec::new();
    // println!("there are some options in this which you can choose from, those options work as their name suggest");

    let mut some_itemslist = load_data();
    // println!("some thing is : {:?}", some_itemslist);

    if some_itemslist.is_empty() {
        println!("there are no items so initialized a new items list, add items to continue");
        execute(&mut some_itemslist);
    //     return;
    } else {
        execute(&mut some_itemslist);
    }

    // create_file();
    //  let some_itemslist = load_data();
    //  println!("some thing is : {:?}", some_itemslist);
}
