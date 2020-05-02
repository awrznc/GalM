//! # GalM
//!
//! [![Actions Status](https://github.com/awrznc/GalM/workflows/Build/badge.svg)](https://github.com/awrznc/GalM/actions)
//! [![Crate](https://img.shields.io/crates/v/galm.svg)](https://crates.io/crates/galm)
//!
//! GalM is pattern matching library.
//!
//! ![galm](https://awrznc.github.io/GalM/assets/image/galm.png)
//!
//! Inspired by Galmoji.
//!
//! ## Quick start
//!
//! Put the following in your project's Cargo.toml file:
//!
//! ```toml
//! [dependencies]
//! galm = "0.0.3"
//! ```
//!
//! And overwrite in your project's main.rs file:
//!
//! ```rust
//! // Get the matching rate.
//! fn main() {
//!
//!     // Initialize GalM Database instance.
//!     let galm: galm::Database = galm::new();
//!
//!     // Get characters similar to the passed in the argument.
//!     let distance: u8 = galm.get_distance("王", "玉");
//!
//!     assert_eq!(distance, 30);
//! }
//! ```
//!
//! Corresponds to the following characters.
//!
//! ```text
//! 一右雨円王音下火花貝
//! 学気九休玉金空月犬見
//! 五口校左三山子四糸字
//! 耳七車手十出女小上森
//! ```
//!
//! ## Example
//!
//! Print the most similar string from the strings separated by commas.
//!
//! ```bash
//! # build
//! cargo build --example galm --release
//!
//! # use galm
//! ./target/release/examples/galm "王様レストラン" --dictionary "皇様レストラン,玉様レストラン,大様レストラン"
//! # => 玉様レストラン
//! ```
//!


use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Character
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Character<'a> {
    pub name: &'a str,
    pub cost: usize,
}

impl Ord for Character<'_> {
    fn cmp(&self, other: &Character) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for Character<'_> {
    fn partial_cmp(&self, other: &Character) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_low_cost<'character>(
    adjacent_list: &'character Characters,
    start: &'character str,
    goal: &str
) -> Option<usize> {

    // set route info
    let mut hash: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for key in adjacent_list.keys() { hash.insert(key, usize::MAX); }

    // set start
    let mut heap = BinaryHeap::new();
    heap.push(Character { cost: 0, name: start });
    hash.insert(start, 0);

    // search shortest distance
    while let Some(Character { cost, name }) = heap.pop() {
        // println!("total_cost: {}, name: {}", cost, name);

        // 最短経路を見つけることができたら終了
        if name == goal { return Some(cost); }

        // すでにより良い経路を見つけていたらスキップ
        if cost > hash[name] { continue; }

        match &adjacent_list.get(name) {
            None => continue,
            Some(adjacent) => {
                for point in adjacent.iter() {
                    let next_point = Character { cost: cost + point.cost, name: point.name };
                    if next_point.cost < hash[next_point.name] {
                        heap.push(next_point);
                        hash.insert(next_point.name, next_point.cost);
                    }
                }
            }
        }
    }
    None
}

macro_rules! json_parse {
    ({ $($key:tt : $tt:tt),+ }) => {
        {
            let mut characters = ::std::collections::HashMap::new();
            $(
                characters.insert($key, array_parse!($tt));
            )+
            characters
        }
    };
}

macro_rules! array_parse {
    ([ $($tt:tt),* ]) => {
        {
            // use if array is not empty
            #[allow(unused_mut)]
            let mut character = Vec::new();
            $(
                character.push( hash_parse!($tt) );
            )*
            character
        }
    };
}

macro_rules! hash_parse {
    ({ $($key:tt : $value:expr),+ }) => {
        {
            let mut node: &str = "";
            let mut cost: usize = 0;
            $(
                match $key {
                    "node" => node = $value,
                    "cost" => cost = $value.parse::<usize>().unwrap(),
                    _ => panic!("Unexpected element."),
                }
            )+
            Character { name: node, cost: cost }
        }
    };
}


/// Database
pub struct Database {
    pub characters: Characters,
    pub max_distance_size: usize
    // pub Idioms: Idioms
}

/// Characters
pub type Characters = std::collections::HashMap<
    &'static str,
    std::vec::Vec<Character<'static>>
>;

/// Database
impl Database {

    /// Initialize GalM Database instance.
    ///
    /// ```rust
    /// let galm: galm::Database = galm::Database::new();
    /// ```
    pub fn new() -> Database {
        return Database {
            characters: include!("./../target/converted_data.rs"),
            max_distance_size: 100
        };
    }

    /// Get the matching rate.
    /// Range is 0..`galm::Database.max_distance_size`.
    /// The more similar the two arguments, the smaller the return value.
    ///
    /// ```rust
    /// // Initialize GalM Database instance.
    /// let galm: galm::Database = galm::Database::new();
    ///
    /// // Get characters similar to the passed in the argument.
    /// let distance: u8 = galm.get_distance("王", "玉");
    ///
    /// assert_eq!(distance, 30);
    /// ```
    pub fn get_distance(&self, from: &str, to: &str) -> u8 {
        return match get_low_cost(&self.characters, from, to) {
            None => self.max_distance_size as u8,
            Some(i) if self.max_distance_size < i => { i as u8 },
            Some(i) => i as u8,
        };
    }
}


/// Initialize GalM Database instance.
///
/// ```rust
/// let galm: galm::Database = galm::new();
/// ```
pub fn new() -> Database {
    return Database::new();
}

pub mod search;
