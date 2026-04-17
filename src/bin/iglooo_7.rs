use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
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

fn read_input() -> Result<String> {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .context("failed to read line")?;
    Ok(user_input.trim().to_string())
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

fn add_item(null_items: &mut Vec<Item>) -> Result<()> {
    println!("enter the product you want to add");
    let product_name = read_input()?;
    let lowercase_productname = product_name.to_lowercase();

    if null_items
        .iter()
        .any(|s| s.name.to_lowercase() == lowercase_productname)
    {
        println!("item already exists");
    } else {
        null_items.push(Item {
            name: product_name,
            status: Status::NotBought,
        });
        save_data(null_items)?;
        println!("item added");
    }
    Ok(())
}

fn remove_item(items_list: &mut Vec<Item>) -> Result<()> {
    if items_list.is_empty() {
        println!("there are no items");
    } else {
        view_item(items_list);
        println!("enter the item name you want to remove");
        let removed_item = read_input()?.to_lowercase();

        if let Some(index) = items_list
            .iter()
            .position(|index| index.name.to_lowercase() == removed_item)
        {
            items_list.remove(index);
            save_data(items_list)?;
            println!("item removed");
        } else {
            println!("no item found");
        }
    }
    Ok(())
}

fn view_item(some_items: &[Item]) {
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

fn buy_items(added_items: &mut Vec<Item>) -> Result<()> {
    if added_items.is_empty() {
        println!("there are no items at this moment");
    } else {
        view_item(added_items);
        println!("enter the item name you want to buy");
        let buying_item = read_input()?.to_lowercase();

        if let Some(item) = added_items
            .iter_mut()
            .find(|item| item.name.to_lowercase() == buying_item)
        {
            item.status = Status::Bought;
            save_data(added_items)?;
            println!("item marked as bought");
        } else {
            println!("item not found");
        }
    }
    Ok(())
}

fn unbought_items(bought_items: &mut Vec<Item>) -> Result<()> {
    if bought_items.is_empty() {
        println!("there are no items");
    } else {
        view_item(bought_items);
        println!("enter the item name you want to unmark");
        let unbuying_item = read_input()?.to_lowercase();

        if let Some(item) = bought_items
            .iter_mut()
            .find(|item| item.name.to_lowercase() == unbuying_item)
        {
            match item.status {
                Status::NotBought => println!("already marked as not bought"),
                Status::Bought => {
                    item.status = Status::NotBought;
                    save_data(bought_items)?;
                    println!("marked as not bought");
                }
            }
        } else {
            println!("item not found");
        }
    }
    Ok(())
}

fn load_data() -> Result<Vec<Item>> {
    match File::open("data.json") {
        Ok(file) => Ok(serde_json::from_reader(BufReader::new(file)).context("failed to parse JSON")?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(anyhow::Error::from(e)),
    }
}

fn save_data(items_list: &[Item]) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("data.json")
        .context("failed to open file")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, items_list).context("failed to write json")?;
    Ok(())
}

fn execute(some_itemslist: &mut Vec<Item>) -> Result<()> {
    loop {
        println!("\n============================================");
        println!("add | buy | unbuy | view | remove | exit");
        println!("============================================");

        let new_input = read_input()?.to_lowercase();
        if new_input == "add" {
            add_item(some_itemslist)?;
        } else if new_input == "buy" {
            buy_items(some_itemslist)?;
        } else if new_input == "view" {
            view_item(some_itemslist);
        } else if new_input == "remove" {
            remove_item(some_itemslist)?;
        } else if new_input == "unbuy" {
            unbought_items(some_itemslist)?;
        } else if new_input == "exit" {
            break;
        } else {
            println!("invalid input");
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut some_itemslist = load_data()?;

    if some_itemslist.is_empty() {
        println!("no items found, starting fresh");
    }

    execute(&mut some_itemslist)
}
