use std::io;

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

const LEGAL_COMMANDS: &'static [&'static str] = &[
    "go", "grab", "move", "pickup", "bite", "hit", "destroy", "shoot", "charge", "attack", "run",
    "jump", "climb",
];

fn is_legal_command<'a>(command_input: &'a str, legal_commands: &[&str]) -> Option<&'a str> {
    if legal_commands.iter().position(|&x| x == command_input) != None {
        Some(command_input)
    } else {
        None
    }
}

struct Exit {
    direction: Direction,
    target: usize,
    locked: bool,
    key: String,
}

impl Exit {
    fn can_go(&self, direction: &Direction) -> bool {
        self.direction == *direction && !self.locked
    }
}

struct Item {
    name: String,
    description: String,
    weight: usize,
}

struct Room {
    description: String,
    exits: Vec<Exit>,
    items: Vec<Item>,
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
    let mut command: Option<String> = None;
    let mut current_room = rooms.first();

    while command == None {
        println!("{}", current_room.unwrap().description);
        println!("\nWhat do you do?\n");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        let mut user_input = input.split_whitespace();

        let predicate = match is_legal_command(user_input.next().unwrap(), LEGAL_COMMANDS) {
            Some(x) => println!("command Result: {}", x),
            None => println!("command result was None"),
        };

        match predicate {
            Some(result) => println!("{}", result),
            None => println!("No words"),
        }
    }
}
