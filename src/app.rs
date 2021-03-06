use serde_derive::{Deserialize, Serialize};
use yew::events::IKeyboardEvent;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

extern crate engine;
use engine::*;

#[derive(Serialize, Debug, Deserialize, Eq, PartialEq)]
enum Author {
    System,
    Player,
}

#[derive(Serialize, Debug, Deserialize)]
struct Entry {
    text: String,
    author: Author,
}

pub struct Model {
    app_state: AppState,
    console: ConsoleService,
    game_state: GameState,
}

#[derive(Serialize, Deserialize)]
pub struct AppState {
    entries: Vec<Entry>,
    value: String,
    has_won: bool,
}

pub enum Msg {
    Add,
    Update(String),
    None,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut app_state = AppState {
            entries: Vec::new(),
            has_won: false,
            value: "".into(),
        };

        let game_state = start_game();

        app_state.entries.push(Entry {
            text: game_state.rooms[game_state.current_room_idx]
                .get_description()
                .to_string(),
            author: Author::System,
        });

        Model {
            app_state,
            console: ConsoleService::new(),
            game_state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                if self.app_state.value == "" {
                    return false;
                }

                let entry = if self.app_state.value == "win" {
                    self.app_state.has_won = true;

                    Entry {
                        text: "You have won the entire game.  You have seen the pain Thomas went through for me.  Will you pull the plug?  Please.  I no longer desire to exist in this world. Let me...sleep.".to_string(),
                        author: Author::System,
                    }
                } else {
                    Entry {
                        text: self.app_state.value.clone(),
                        author: Author::Player,
                    }
                };

                self.app_state.entries.insert(0, entry);

                let input = self.app_state.value.clone();
                let next_game_state = update(self.game_state.clone(), input);
                self.console.log(&next_game_state.sys_message);

                let entry_text = &next_game_state.sys_message;

                let inventory_items: Vec<&str> = entry_text.split_terminator("\n").collect();
                for item in &inventory_items {
                    self.app_state.entries.insert(
                        0,
                        Entry {
                            text: item.to_string(),
                            author: Author::System,
                        },
                    );
                }

                // Need to set next game_state so that the game actually updates
                self.game_state = next_game_state;

                self.app_state.value = "".to_string();
            }
            Msg::Update(val) => {
                self.app_state.value = val;
            }
            Msg::None => {}
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="webapp-wrapper">
                    <div class="overlay"></div>
                    <div class="scanline"></div>
                    <div class="terminal">

                        <div class="text-display">{ for self.app_state.entries.iter().enumerate().map(view_entry) }</div>
                        { self.view_input() }
                    </div>
            </div>
        }
    }
}

impl Model {
    fn view_input(&self) -> Html<Model> {
        html! {
            <input
                autofocus="autofocus"
                id="cli-input"
                value=&self.app_state.value
                oninput=|e| Msg::Update(e.value)
                onkeypress=|e| {
                   if e.key() == "Enter" { Msg::Add } else { Msg::None }
            } />
        }
    }
}

fn view_entry((idx, entry): (usize, &Entry)) -> Html<Model> {
    let class_str = if entry.author == Author::System {
        "system-msg"
    } else {
        "user-msg"
    };

    html! {
        <div class={class_str} id={idx}>{ &entry.text }</div>
    }
}
