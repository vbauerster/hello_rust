#![allow(unused)]

use std::marker::PhantomData;

struct Alive;
struct Dead;

trait PlayerState {}

impl PlayerState for Alive {}
impl PlayerState for Dead {}

// or `Player<State: PlayerState = Alive>`
struct Player<State = Alive>
where
    State: PlayerState,
{
    username: String,
    health: u8,
    items: Vec<String>,
    state: PhantomData<State>,
}

impl Player<Alive> {
    pub fn die(self) -> Player<Dead> {
        Player {
            username: self.username,
            health: 0,
            items: self.items,
            state: PhantomData,
        }
    }

    pub fn sell_item(&mut self, item_idx: usize) {
        self.items.remove(item_idx);
    }

    pub fn buy_item(&mut self, item: String) {
        self.items.push(item);
    }
}

impl Player<Dead> {
    pub fn get_resurrected(self) -> Player<Alive> {
        Player {
            username: self.username,
            health: 100,
            items: self.items,
            state: PhantomData,
        }
    }
}

impl<State: PlayerState> Player<State> {
    pub fn list_items(&self) -> &[String] {
        self.items.as_slice()
    }
}

impl Player {
    pub fn new(username: String) -> Self {
        Player {
            username,
            health: 100,
            items: Default::default(),
            state: PhantomData,
        }
    }
}

fn main() {
    let mut player = Player::new("HiMom".to_owned()); // create an `Alive` player
    player.buy_item("item".to_owned());

    /* the below commented out code creates compile time error */
    /*****************/
    // player.get_resurrected(); // no method named `get_resurrected()` found for struct `Player` in the current scope

    let player = player.die(); // we can kill a an `Alive` player

    /*  CANNOT kill what's already dead */
    /*****************/
    // player.kill(); // no method named `kill` found for struct `Player<Dead>` in the current scope

    // can list items for `Alive` or `Dead` player
    for item in player.list_items() {
        println!("dead: {item}");
    }
    // can resurrect a `Dead` player
    let mut player = player.get_resurrected();
    // can list items for `Alive` or `Dead` player
    for item in player.list_items() {
        println!("alive: {item}");
    }

    /* we can ensure the TraitBound also in compile-time, the below code creates the error: */
    /*  the trait bound `u8: PlayerState` is not satisfied */
    /*****************/
    // let invalid_player = Player::<u8> {
    //     username: "potato".to_string(),
    //     health: 100,
    //     items: vec!["frostmourne".to_string()],
    //     state: PhantomData,
    // };
}
