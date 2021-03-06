use std::collections::HashMap;

#[path = "commands.rs"]
mod commands;

#[path = "direction.rs"]
mod direction;

#[path = "examine.rs"]
mod examine;

#[path = "item.rs"]
mod item;

use commands::*;
use direction::*;
use examine::*;
use item::*;

#[derive(Clone, Debug)]
struct Exit {
    direction: Direction,
    locked: bool,
    interactable_id: String,
    target: usize,
}

impl Exit {
    fn is_locked(&self) -> bool {
        self.locked
    }

    fn unlock(&mut self) {
        self.locked = false
    }
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub current_room_idx: usize,
    pub inventory: HashMap<&'static str, Item>,
    pub sys_message: String,
    pub rooms: Vec<Room>,
}

#[derive(Debug, Default)]
struct Input {
    intent: Intent,
    is_direction: bool,
    is_interactable: bool,
    is_item: bool,
    object_noun: String,
}

#[derive(Clone, Debug)]
struct Interactable {
    after_interaction_description: &'static str,
    before_interaction_description: &'static str,
    id: String,
    interaction_description: &'static str,
    interacted: bool,
    name: String,
    prerequisite_item: String,
}

impl Interactable {
    fn interact(&mut self) {
        self.interacted = true
    }

    fn is_interacted(&self) -> bool {
        self.interacted
    }
}

impl Examine for Interactable {
    fn examine(&self) -> &'static str {
        if self.interacted {
            self.after_interaction_description
        } else {
            self.before_interaction_description
        }
    }
}

#[derive(Clone, Debug)]
pub struct Room {
    description: String,
    interactables: Vec<Interactable>,
    items: Vec<&'static str>,
    exits: Vec<Exit>,
}

impl Room {
    pub fn get_description(&self) -> &str {
        &self.description
    }
}

pub fn start_game() -> GameState {
    let rooms = vec![
            Room {
                description: "A wind blows over the dunes of sand that cover the known world as you step up to a large dilapidated building.

Unlike other ruins you have seen in the past, this structure does not speak of a lavish past.

You are greeted with a metal door weathered from the years and bearing a strange insignia.".to_string(),
                exits: vec![
                    Exit {
                        direction: Direction::S,
                        interactable_id: "lab_entrance".to_string(),
                        locked: true,
                        target: 2,
                    },
                ],
                interactables: vec![Interactable{id: "lab_entrance".to_string(), name: "door".to_string(), interaction_description: "The pendant fits into the panel in the door.
You hear a brief beeping sound and see a few lights on the panel turn from red to green.
The door swings open to the south.", before_interaction_description: "You notice a small panel to the side of the door with what seems to be a slot to fit something in.", after_interaction_description: "The door has slid open and exposed a path to the south.", interacted: false, prerequisite_item: "pendant".to_string()}],
                items: vec![],
            },
            Room {
                description: "You find yourself in a room. There is a door to the west and a door to the south. You notice a small crevice in the corner.  The room with the helmet".to_string(),
                exits: vec![
                    Exit {
                        direction: Direction::W,
                        interactable_id: "".to_string(),
                        locked: false,
                        target: 0,
                    },
                    Exit {
                        direction: Direction::S,
                        interactable_id: "".to_string(),
                        locked: false,
                        target: 3,
                    },
                ],
                interactables: vec![],
                items: vec!["helmet"],
            },
            Room {
                description: "You find yourself in a room. There is a door to the north".to_string(),
                exits: vec![
                    Exit {
                        direction: Direction::N,
                        interactable_id: "".to_string(),
                        target: 0,
                        locked: false,
                    },
                ],
                interactables: vec![],
                items: vec![],
            },
            Room {
                description: "You find yourself in a room. There is a door to the north. The door to the south is locked.".to_string(),
                exits: vec![
                    Exit {
                        direction: Direction::N,
                        interactable_id: "".to_string(),
                        target: 1,
                        locked: false,
                    },
                    Exit {
                        direction: Direction::S,
                        interactable_id: "".to_string(),
                        target: 4,
                        locked: false,
                    },
                ],
                interactables: vec![],
                items: vec![],
            },
            Room {
                description: "Dungeon exit".to_string(),
                exits: vec![],
                interactables: vec![],
                items: vec![],
    }
        ];

    return GameState {
        current_room_idx: 0,
        inventory: create_inventory(),
        rooms: rooms,
        sys_message: "".to_string(),
    };
}

pub fn update(prev_state: GameState, input: String) -> GameState {
    let mut parsed_input = Input {
        ..Default::default()
    };

    let mut new_game_state = prev_state.clone();

    // use the new_game_state instead of previous so that we modify the new_game_state when
    // interacting.
    // This is fine since we just have a cloned previous state here
    let room = &mut new_game_state.rooms[new_game_state.current_room_idx];
    let user_inventory = &mut new_game_state.inventory;

    let mut user_input = input.split_whitespace().peekable();
    let first_command = user_input.next().unwrap();

    if !is_legal_command(first_command) {
        // If the command is not valid, we do not need to parse the rest of the string input
        new_game_state.sys_message = format!("{} is not a legal command\n", first_command);
        return new_game_state;
    };

    parsed_input.intent = determine_intent(first_command).unwrap();

    for word in user_input {
        let lowercase_word = word.to_lowercase();
        if parsed_input.object_noun == "" {
            if lowercase_word == "inventory" {
                parsed_input.object_noun = lowercase_word;
                continue;
            }

            if is_direction(lowercase_word.as_str()) {
                parsed_input.object_noun = lowercase_word;
                parsed_input.is_direction = true;
                continue;
            }

            if user_inventory.contains_key(lowercase_word.as_str()) {
                parsed_input.object_noun = lowercase_word;
                parsed_input.is_item = true;
                continue;
            }

            if room.interactables.iter().any(|x| x.name == lowercase_word) {
                parsed_input.object_noun = lowercase_word;
                parsed_input.is_interactable = true;
                continue;
            }
        }
    }

    if parsed_input.object_noun.is_empty() {
        new_game_state.sys_message = format!("I was unable to understand your command.  Please re-enter and try again.")    
    }

    match parsed_input.intent {
        Intent::EXAMINE => {
            if parsed_input.is_interactable {
                let description = room
                    .interactables
                    .iter()
                    .find(|x| x.name == parsed_input.object_noun)
                    .unwrap()
                    .examine();

                new_game_state.sys_message = description.to_string();
            } else if parsed_input.is_item {
                let description = user_inventory
                    .get::<str>(&parsed_input.object_noun)
                    .unwrap()
                    .get_description();
                new_game_state.sys_message = description.to_string();
            }
        }
        Intent::INTERACT => {
            if parsed_input.is_interactable {
                let inter_pos = room
                    .interactables
                    .iter()
                    .position(|x| x.name == parsed_input.object_noun)
                    .unwrap();

                new_game_state.sys_message = match room.interactables.get_mut(inter_pos) {
                    Some(x) => {
                        if x.prerequisite_item.is_empty() {
                            x.interact();
                            x.interaction_description.to_string()
                        } else {
                            format!(
                                "You currently can not interact with {}",
                                room.interactables[inter_pos].name.clone()
                            )
                        }
                    },
                    None => { 
                        format!("There is no {} in this room", parsed_input.object_noun) 
                    },
                }
            }
        }
        Intent::INVENTORY => {
            let key = &parsed_input.object_noun;
            if parsed_input.is_item {
                let item = user_inventory.get_mut::<str>(key).unwrap();
                if *item.get_location() == ItemState::Room {
                    item.to_inventory();

                    new_game_state.sys_message =
                        format!("You have picked up a {}", item.get_name().to_string());
                } else {
                    new_game_state.sys_message =
                        format!("You already have the {}", item.get_name().to_string());
                }
            }
        }
        Intent::LIST_INVENTORY => {
            let initial_msg = "Your inventory:\n";
            let mut inventory_message: String = "".to_string();
            inventory_message.push_str(initial_msg);
            for item in user_inventory.values() {
                // If the item isn't in the Room, it is either in the user's inventory or equipped
                // since there are currently only three states
                if *item.get_location() != ItemState::Room {
                    inventory_message.push_str(&format!(
                        "{}: {}\n",
                        item.get_name(),
                        item.get_description()
                    ));
                }
            }

            new_game_state.sys_message = match inventory_message == initial_msg {
                true => "You have no items in your inventory".to_string(),
                false => inventory_message,
            };
        }
        Intent::MOVEMENT => {
            if parsed_input.is_direction {
                let direction: Direction = text_to_direction(&parsed_input.object_noun).unwrap();

                let exit: Option<&Exit> = room.exits.iter().find(|&x| x.direction == direction);

                if exit.is_none() {
                    new_game_state.sys_message =
                        format!("There is no exit leaving {}", parsed_input.object_noun);
                } else if exit.unwrap().is_locked() {
                    new_game_state.sys_message =
                        format!("The way is locked. You must unlock the path before you proceed.");
                } else if parsed_input.is_direction && exit.is_some() {
                    new_game_state.current_room_idx = exit.unwrap().target;
                    new_game_state.sys_message = new_game_state.rooms
                        [new_game_state.current_room_idx]
                        .description
                        .to_string();
                }
            } else if !parsed_input.is_direction {
                new_game_state.sys_message =
                    format!("There is no path to the {}", parsed_input.object_noun);
            }
        }
        Intent::USE => { 
            let is_in_inventory = match user_inventory.get::<str>(&parsed_input.object_noun) {
                Some(x) => x.is_in_inventory(),
                None => false,
            };


            let inter_pos = room
                    .interactables
                    .iter()
                    .position(|x| x.prerequisite_item == parsed_input.object_noun);

            // is_some check is used to ensure that the interactable is actually in this room
            new_game_state.sys_message = if parsed_input.is_item && is_in_inventory && inter_pos.is_some() {
                match room.interactables.get_mut(inter_pos.unwrap()) {
                    Some(x) => {
                        if x.is_interacted() {
                            format!("{} has already been used here", x.prerequisite_item)
                        } else {
                            // unlock room if it is dependent on the interactable_id
                            let room_pos = room.exits.iter().position(|exit| exit.interactable_id == x.id).unwrap();
                            match room.exits.get_mut(room_pos) {
                                Some(exit) => exit.unlock(),
                                None => println!("silently do not unlock the exit"),
                            };
                            x.interact(); 
                            // set the item to the room because it has been used and can not be
                            // used again
                            user_inventory.get_mut::<str>(&parsed_input.object_noun).unwrap().to_room();
                            x.interaction_description.to_string()
                        }
                    },
                    None => format!("There is no use for the item {} in this room", parsed_input.object_noun),
                }
            // check for if the item is in your inventory first in order to not let the player know 
            // the item is required here if they don't have the item
            } else if !is_in_inventory {
                format!("You have no item of that name in your inventory")
            } else {
                format!("You can not use `{}` here", parsed_input.object_noun)
            };
        },
        _ => new_game_state.sys_message = format!("You didn't choose an appropriate command"),
    }

    return new_game_state;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_inventory() -> HashMap<&'static str, Item> {
    let mut map = HashMap::new();

    map.insert(
        "helmet",
        Item {
            name: "helmet".to_string(),
            description: "a blue helmet covered in dirt".to_string(),
            location: ItemState::Room,
        },
    );

    map.insert(
        "buster",
        Item {
            name: "buster".to_string(),
            description: "A large cannon with four buttons".to_string(),
            location: ItemState::Room,
        },
    );

    map.insert(
        "pendant",
        Item {
            name: "pendant".to_string(),
            description: "A rusty pendant with a small seal on it.".to_string(),
            location: ItemState::Inventory,
        },
    );

    map
}

    fn create_base_game_state() -> GameState {
        let rooms = vec![Room {
            description: "Test Room 1".to_string(),
            exits: vec![Exit {
                direction: Direction::S,
                interactable_id: "".to_string(),
                target: 1,
                locked: false,
            }],
            interactables: vec![],
            items: vec!["helmet"],
        }];

        GameState {
            current_room_idx: 0,
            inventory: create_test_inventory(),
            sys_message: "".to_string(),
            rooms: rooms,
        }
    }

    #[test]
    fn test_locked_exit() {
        let new_inter = Interactable {
            after_interaction_description: "The stone is sitting on the floor",
            before_interaction_description: "You see a stone sitting in between two logs",
            id: "lab_stone".to_string(),
            interacted: false,
            interaction_description: "The stone rolls onto the floor",
            name: "stone".to_string(),
            prerequisite_item: "".to_string(),
        };

        let rooms = vec![Room {
            description: "Test Room 1".to_string(),
            exits: vec![Exit {
                direction: Direction::S,
                interactable_id: "".to_string(),
                target: 1,
                locked: true,
            }],
            interactables: vec![new_inter],
            items: vec![],
        }];

        let game_state = GameState {
            current_room_idx: 0,
            inventory: create_test_inventory(),
            sys_message: "".to_string(),
            rooms: rooms,
        };

        let before_state = update(game_state, "go south".to_string());

        assert_eq!("The way is locked. You must unlock the path before you proceed.".to_string(), before_state.sys_message);
    }

    #[test]
    fn test_use_to_unlock() {
       let new_inter = Interactable {
           after_interaction_description: "The stone is sitting on the floor",
            before_interaction_description: "You see a stone sitting in between two logs",
            id: "lab_stone".to_string(),
            interacted: false,
            interaction_description: "The stone rolls onto the floor",
            name: "stone".to_string(),
            prerequisite_item: "helmet".to_string(),
       };

        let new_item = Item {
            name: "helmet".to_string(),
            description: "A large, blue helmet".to_string(),
            location: ItemState::Inventory,
        };

        let rooms = vec![
            Room {
                description: "Test Room 1".to_string(),
                exits: vec![Exit {
                    direction: Direction::S,
                    interactable_id: "lab_stone".to_string(),
                    target: 1,
                    locked: true,
                }],
                interactables: vec![new_inter],
                items: vec!["helmet"],
            },
            Room {
                description: "Test Room 2".to_string(),
                exits: vec![Exit {
                    direction: Direction::N,
                    interactable_id: "".to_string(),
                    target: 0,
                    locked: false,
                }],
                interactables: vec![],
                items: vec![],
            }
        ];

        let mut inventory = create_test_inventory();
        inventory.get_mut("helmet").unwrap().location = ItemState::Inventory;

        let game_state = GameState {
            current_room_idx: 0,
            inventory: inventory,
            sys_message: "".to_string(),
            rooms: rooms,
        };

        let expected_after_description = "The stone rolls onto the floor";

        let after_interacted_state = update(game_state, "use helmet".to_string());

        assert_eq!(after_interacted_state.sys_message, expected_after_description);

        // ensure that the location is changed after using the helmet
        assert_eq!(after_interacted_state.inventory.get("helmet").unwrap().get_location(), &ItemState::Room);
    }

    #[test]
    fn test_interact() {
        let new_inter = Interactable {
            after_interaction_description: "The stone is sitting on the floor",
            before_interaction_description: "You see a stone sitting in between two logs",
            id: "lab_stone".to_string(),
            interacted: false,
            interaction_description: "The stone rolls onto the floor",
            name: "stone".to_string(),
            prerequisite_item: "".to_string(),
        };

        let rooms = vec![Room {
            description: "Test Room 1".to_string(),
            exits: vec![Exit {
                direction: Direction::S,
                interactable_id: "".to_string(),
                target: 1,
                locked: false,
            }],
            interactables: vec![new_inter],
            items: vec![],
        }];

        let game_state = GameState {
            current_room_idx: 0,
            inventory: create_test_inventory(),
            sys_message: "".to_string(),
            rooms: rooms,
        };

        let expected_after_interactable_description = "The stone is sitting on the floor";
        let expected_before_interactable_description =
            "You see a stone sitting in between two logs";
        let expected_interaction_description = "The stone rolls onto the floor";

        let before_state = update(game_state, "examine stone".to_string());
        let interacting_state = update(before_state.clone(), "push stone".to_string());
        let after_state = update(interacting_state.clone(), "examine stone".to_string());

        assert_eq!(
            expected_before_interactable_description,
            before_state.sys_message
        );
        assert_eq!(
            expected_interaction_description,
            interacting_state.sys_message
        );
        assert_eq!(
            expected_after_interactable_description,
            after_state.sys_message
        );
    }

    #[test]
    fn test_no_interactable() {
        let new_inter = Interactable {
            after_interaction_description: "The stone is sitting on the floor",
            before_interaction_description: "You see a stone sitting in between two logs",
            id: "lab_stone".to_string(),
            interacted: false,
            interaction_description: "The stone rolls onto the floor",
            name: "stone".to_string(),
            prerequisite_item: "".to_string(),
        };

        let rooms = vec![Room {
            description: "Test Room 1".to_string(),
            exits: vec![Exit {
                direction: Direction::S,
                interactable_id: "".to_string(),
                target: 1,
                locked: false,
            }],
            interactables: vec![new_inter],
            items: vec![],
        }];

        let game_state = GameState {
            current_room_idx: 0,
            inventory: create_test_inventory(),
            sys_message: "".to_string(),
            rooms: rooms,
        };

        let expected_interactable_description = "I was unable to understand your command.  Please re-enter and try again.";
        let after_state = update(game_state, "examine face".to_string());

        assert_eq!(
            expected_interactable_description,
            after_state.sys_message
        );
    }

    #[test]
    fn test_move() {
        let rooms = vec![
            Room {
                description: "Test Room 1".to_string(),
                exits: vec![Exit {
                    direction: Direction::S,
                    interactable_id: "".to_string(),
                    target: 1,
                    locked: false,
                }],
                interactables: vec![],
                items: vec![],
            },
            Room {
                description: "Test Room 2".to_string(),
                exits: vec![Exit {
                    direction: Direction::N,
                    interactable_id: "".to_string(),
                    target: 0,
                    locked: false,
                }],
                interactables: vec![],
                items: vec![],
            },
        ];

        let game_state = GameState {
            current_room_idx: 0,
            inventory: create_test_inventory(),
            sys_message: "".to_string(),
            rooms: rooms,
        };

        let next_game_state = update(game_state, "go south".to_string());

        let expected_room_idx = 1;
        let expected_sys_message = "Test Room 2";

        assert_eq!(expected_room_idx, next_game_state.current_room_idx);
        assert_eq!(expected_sys_message, next_game_state.sys_message);
    }

    #[test]
    fn test_update_inventory() {
        let new_item = Item {
            name: "helmet".to_string(),
            description: "A large, blue helmet".to_string(),
            location: ItemState::Room,
        };

        let rooms = vec![Room {
            description: "Test Room 1".to_string(),
            exits: vec![Exit {
                direction: Direction::S,
                interactable_id: "".to_string(),
                target: 1,
                locked: false,
            }],
            interactables: vec![],
            items: vec!["helmet"],
        }];

        let game_state = GameState {
            current_room_idx: 0,
            inventory: create_test_inventory(),
            sys_message: "".to_string(),
            rooms: rooms,
        };

        let before_state = update(game_state.clone(), "grab helmet".to_string());
        let after_state = update(before_state.clone(), "grab helmet".to_string());

        let expected_before_sys_message = "You have picked up a helmet";
        let expected_sys_message = "You already have the helmet";

        assert_eq!(expected_before_sys_message, before_state.sys_message);
        assert_eq!(expected_sys_message, after_state.sys_message);
    }

    #[test]
    fn test_list_inventory() {
        let rooms = vec![Room {
            description: "Test Room 1".to_string(),
            exits: vec![Exit {
                direction: Direction::S,
                interactable_id: "".to_string(),
                target: 1,
                locked: false,
            }],
            interactables: vec![],
            items: vec!["helmet"],
        }];

        let game_state = GameState {
            current_room_idx: 0,
            inventory: create_test_inventory(),
            sys_message: "".to_string(),
            rooms: rooms,
        };

        let before_state = update(game_state.clone(), "grab helmet".to_string());
        let new_game_state = update(before_state.clone(), "list inventory".to_string());

        let expected_sys_message = "Your inventory:\nhelmet: a blue helmet covered in dirt\npendant: A rusty pendant with a small seal on it.\n";

        assert!(new_game_state.sys_message.contains("helmet: a blue helmet") && new_game_state.sys_message.contains("pendant: A rusty pendant"));
    }

    #[test]
    fn test_empty_inventory_display_diff_message() {
        let mut game_state = create_base_game_state();
        let has_inventory_state = update(game_state.clone(), "list inventory".to_string());

        assert!(!has_inventory_state.sys_message.contains("You have no items in your inventory"));
        assert!(has_inventory_state.sys_message.contains("rusty pendant with a small seal"));

        game_state.inventory = HashMap::new();

        let no_inventory_state = update(game_state.clone(), "list inventory".to_string());

        assert!(no_inventory_state.sys_message.contains("You have no items in your inventory"));
    }
}
