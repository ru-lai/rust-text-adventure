use std::collections::HashMap;
use std::io;

mod commands;

use crate::commands::*;

#[derive(PartialEq)]
enum Command {
    Go(Direction),
    Unlock(Direction),
    Interact(String),
}

#[derive(PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

fn is_legal_command<'a>(command_input: &'a str, legal_commands: &[&str]) -> Option<&'a str> {
    if legal_commands.iter().position(|&x| x == command_input) != None {
        Some(command_input)
    } else {
        None
    }
}

fn is_number(input: &str) -> bool {
    return input.parse::<i32>().is_ok();
}

fn is_obj_noun<'a>(word: &'a str, item_map: &HashMap<String, Item>) -> bool {
    return item_map.contains_key(word);
}

struct Exit {
    direction: Direction,
    target: i32,
    locked: bool,
    key: String,
}

impl Exit {
    fn can_go(&self, direction: &Direction) -> bool {
        self.direction == *direction && !self.locked
    }
}

#[derive(Debug, Default)]
struct Input {
    intent: String,
    number_of: i32,
    object_noun: String,
}

trait Equippable {
    fn is_equipped(&self) {}
}

#[derive(Debug)]
enum ItemState {
    Room,
    Inventory,
    Equipped,
}

#[derive(Debug)]
struct Item {
    name: String,
    description: String,
    weight: i32,
    location: ItemState,
}

fn get_item_vec() -> Vec<(String, Item)> {
    return vec![
        (
            "helmet".to_string(),
            Item {
                name: "helmet".to_string(),
                description: "A blue helmet covered in dirt".to_string(),
                weight: 30,
                location: ItemState::Room,
            },
        ),
        (
            "buster".to_string(),
            Item {
                name: "buster".to_string(),
                description: "A large cannon with four buttons".to_string(),
                weight: 20,
                location: ItemState::Room,
            },
        ),
    ];
}

impl Item {
    fn new(name: String, description: String, weight: i32) -> Self {
        Item {
            name: name,
            description: description,
            weight: weight,
            location: ItemState::Room,
        }
    }

    fn to_inventory(&mut self) {
        self.location = match self.location {
            _ => ItemState::Inventory,
        }
    }

    fn equip(&mut self) {
        self.location = match self.location {
            ItemState::Room => ItemState::Room,
            _ => ItemState::Equipped,
        }
    }

    fn unequip(&mut self) {
        self.location = match self.location {
            ItemState::Room => ItemState::Room,
            _ => ItemState::Inventory,
        }
    }
}

struct Room {
    description: String,
    items: Vec<Item>,
    exits: Vec<Exit>,
}

fn main() {
    let mut rooms = vec![
            Room {
                description: "You find yourself in a room. There is a door to the south and a door to the east.".to_string(),
                exits: vec![
                    Exit {
                        direction: Direction::S,
                        target: 2,
                        locked: false,
                        key: String::from(""),
                    },
                    Exit {
                        direction: Direction::E,
                        target: 1,
                        locked: false,
                        key: String::from(""),
                    },
                ],
                items: vec![],
            },
            Room {
                description: "You find yourself in a room. There is a door to the west and a door to the south.".to_string(),
                exits: vec![
                    Exit {
                        direction: Direction::W,
                        target: 0,
                        locked: false,
                        key: String::from(""),
                    },
                    Exit {
                        direction: Direction::S,
                        target: 3,
                        locked: false,
                        key: String::from(""),
                    },
                ],
                items: vec![],
            },
            Room {
                description: "You find yourself in a room. There is a door to the north. A key is here.".to_string(),
                exits: vec![
                    Exit {
                        direction: Direction::N,
                        target: 0,
                        locked: false,
                        key: String::from(""),
                    },
                ],
                items: vec![],
            },
            Room {
                description: "You find yourself in a room. There is a door to the north. The door to the south is locked.".to_string(),
                exits: vec![
                    Exit {
                        direction: Direction::N,
                        target: 1,
                        locked: false,
                        key: String::from(""),
                    },
                    Exit {
                        direction: Direction::S,
                        target: 4,
                        locked: true,
                        key: String::from(""),
                    },
                ],
                items: vec![],
            },
            Room {
                description: "Dungeon exit".to_string(),
                exits: vec![],
                items: vec![],
    }
        ];
    let mut movement: Option<String> = None;
    let mut current_room = rooms.first();
    let mut parsed_input = Input {
        ..Default::default()
    };

    let item_vec = get_item_vec();
    let inventory_map: HashMap<_, _> = item_vec.into_iter().collect();

    while movement == None {
        println!("{}", current_room.unwrap().description);
        println!("\nWhat do you do?\n");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        let mut user_input = input.split_whitespace().peekable();

        // transform input to parsed form

        let intent = user_input.next().unwrap();

        if is_legal_command(intent, commands::legal_commands::LEGAL_COMMANDS) == None {
            println!("{} is not a legal command\n", intent);
            continue;
        };

        parsed_input.intent = intent.to_string();
        for word in user_input {
            if is_number(word) {
                parsed_input.number_of = word.parse::<i32>().unwrap();
                continue;
            }

            if inventory_map.contains_key(word) {
                parsed_input.object_noun = word.to_string();
            }
        }
    }
}
