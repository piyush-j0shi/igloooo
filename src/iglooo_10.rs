use std::collections::HashMap;
use std::hash::Hash;
use std::io;

type RoomId = String;
type ItemId = String;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Direction {
    fn from_str(input: &str) -> Option<Direction> {
        match input.to_lowercase().as_str() {
            "north" => Some(Direction::North),
            "south" => Some(Direction::South),
            "east" => Some(Direction::East),
            "west" => Some(Direction::West),
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Room {
    id: RoomId,
    name: String,
    description: String,
    exits: HashMap<Direction, RoomId>,
    items: Vec<ItemId>,
}

#[derive(Debug)]
struct Item {
    id: ItemId,
    name: String,
    description: String,
}

#[derive(Debug)]
struct Player {
    currentroom: RoomId,
    inventory: Vec<ItemId>,
}

struct GameState {
    rooms: HashMap<RoomId, Room>,
    items: HashMap<ItemId, Item>,
    player: Player,
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn create_world() -> GameState {
    let item1 = Item {
        id: "item1".to_string(),
        name: "Key".to_string(),
        description: "A small rusty key.".to_string(),
    };
    let item2 = Item {
        id: "item2".to_string(),
        name: "Lantern".to_string(),
        description: "A lantern with some oil left.".to_string(),
    };
    let item3 = Item {
        id: "item3".to_string(),
        name: "Coin".to_string(),
        description: "A shiny gold coin.".to_string(),
    };

    let mut exit1 = HashMap::new();
    exit1.insert(Direction::East, "room2".to_string());

    let mut exit2 = HashMap::new();
    exit2.insert(Direction::South, "room3".to_string());
    exit2.insert(Direction::West, "room1".to_string());

    let mut exit3 = HashMap::new();
    exit3.insert(Direction::North, "room2".to_string());

    let room1 = Room {
        id: "room1".to_string(),
        name: "Entrance".to_string(),
        description: "You are at the entrance of a dark cave.".to_string(),
        exits: exit1,
        items: vec!["item1".to_string()],
    };

    let room2 = Room {
        id: "room2".to_string(),
        name: "Main Hall".to_string(),
        description: "A large hall with echoes.".to_string(),
        exits: exit2,
        items: vec!["item2".to_string()],
    };

    let room3 = Room {
        id: "room3".to_string(),
        name: "Treasure Room".to_string(),
        description: "You see a chest of gold.".to_string(),
        exits: exit3,
        items: vec!["item3".to_string()],
    };

    let player = Player {
        currentroom: "room1".to_string(),
        inventory: vec![],
    };

    let mut rooms = HashMap::new();
    rooms.insert("room1".to_string(), room1);
    rooms.insert("room2".to_string(), room2);
    rooms.insert("room3".to_string(), room3);

    let mut items = HashMap::new();
    items.insert("item1".to_string(), item1);
    items.insert("item2".to_string(), item2);
    items.insert("item3".to_string(), item3);

    GameState {
        rooms,
        items,
        player,
    }
}

fn describe_room(room: &Room, items: &HashMap<ItemId, Item>) {
    println!("\n== {} ==", room.name);
    println!("{}", room.description);

    if !room.items.is_empty() {
        println!("You see:");
        for item_id in &room.items {
            if let Some(item) = items.get(item_id) {
                println!("- {}: {}", item.name, item.description);
            }
        }
    }

    if !room.exits.is_empty() {
        println!("Exits:");
        for (dir, _) in &room.exits {
            println!("- {:?}", dir);
        }
    }
}

fn move_player(gamestate: &mut GameState, dir: &str) {
    if let Some(direction) = Direction::from_str(dir) {
        if let Some(current_room) = gamestate.rooms.get(&gamestate.player.currentroom) {
            if let Some(next_room_id) = current_room.exits.get(&direction) {
                gamestate.player.currentroom = next_room_id.clone();
                println!("You moved to {}", next_room_id);
            } else {
                println!("You can't go that way.");
            }
        }
    } else {
        println!("Invalid direction.");
    }
}

fn take_item(gamestate: &mut GameState, item_name: &str) {
    if let Some(current_room) = gamestate.rooms.get_mut(&gamestate.player.currentroom) {
        if let Some(index) = current_room.items.iter().position(|id| {
            gamestate.items.get(id).map_or(false, |item| {
                item.name.to_lowercase() == item_name.to_lowercase()
            })
        }) {
            let item_id = current_room.items.remove(index);
            gamestate.player.inventory.push(item_id.clone());
            let item = &gamestate.items[&item_id];
            println!("You picked up: {}", item.name);
        } else {
            println!("No such item here.");
        }
    }
}

fn show_inventory(gamestate: &GameState) {
    println!("\nInventory:");
    if gamestate.player.inventory.is_empty() {
        println!("You are not carrying anything.");
    } else {
        for item_id in &gamestate.player.inventory {
            if let Some(item) = gamestate.items.get(item_id) {
                println!("- {}: {}", item.name, item.description);
            }
        }
    }
}

fn game_loop(mut gamestate: GameState) {
    loop {
        let current_room = gamestate.rooms.get(&gamestate.player.currentroom).unwrap();
        describe_room(current_room, &gamestate.items);

        println!("\nEnter command (look, go <dir>, take <item>, inventory, quit):");
        let input = read_input();
        let mut parts = input.split_whitespace();
        let command = parts.next();

        match command {
            Some("look") => {}
            Some("go") => {
                if let Some(dir) = parts.next() {
                    move_player(&mut gamestate, dir);
                } else {
                    println!("Go where?");
                }
            }
            Some("take") => {
                if let Some(item) = parts.next() {
                    take_item(&mut gamestate, item);
                } else {
                    println!("Take what?");
                }
            }
            Some("inventory") => {
                show_inventory(&gamestate);
            }
            Some("quit") => {
                println!("Thanks for playing!");
                break;
            }
            _ => println!("Unknown command."),
        }
    }
}

fn main() {
    let gamestate = create_world();
    game_loop(gamestate);
}
