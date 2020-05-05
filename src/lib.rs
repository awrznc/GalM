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
//! galm = "0.0.5"
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


macro_rules! load_json {
    ($path:tt) => {
        {
            let json_string: Vec<&str> = include_str!($path).split('"').collect();

            let mut strings: Vec<&str> = Vec::new();
            for (i, val) in json_string.iter().enumerate() {
                if (i + 1) % 2 == 0 {
                    strings.push(val);
                }
            }

            let mut characters: Characters = std::collections::HashMap::new();
            let mut hash_key: &str = "";
            let mut node: &str = "";
            let mut cost: usize = 0;
            let node_str: &str = "node";
            let cost_str: &str = "cost";
            let mut is_key: bool = true;
            let mut is_align: (bool, bool) = (false, false);
            let mut is_node: bool = false;
            let mut is_cost: bool = false;
            let mut char_vec: Vec<Character> = Vec::new();

            for string in strings.iter() {
                if is_key == true {
                    is_key = false;
                    hash_key = string;
                } else if string == &node_str {
                    is_node = true;
                } else if string == &cost_str {
                    is_cost = true;
                } else if is_node == true {
                    node = string;
                    is_align.0 = true;
                    is_node = false;
                } else if is_cost == true {
                    cost = string.parse::<usize>().unwrap();
                    is_align.1 = true;
                    is_cost = false;
                } else {
                    characters.insert(hash_key, char_vec.clone());
                    char_vec = Vec::new();
                    hash_key = string;
                    continue;
                }
                if is_align == (true, true) {
                    char_vec.push(Character { name: node, cost: cost });
                    is_align = (false, false);
                }
            }

            characters.insert(hash_key, char_vec.clone());

            characters
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
            characters: load_json!("./../docs/assets/json/characters.json"),
            max_distance_size: 100
        };
    }

    /// Get the matching rate fo character.
    /// Range is 0..[`galm::Database.max_distance_size`](#structfield.max_distance_size).
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

    /// Get the matching rate of word.
    /// Range is 0..[`std::usize::MAX`](https://doc.rust-lang.org/std/usize/constant.MAX.html).
    /// The more similar the two arguments, the smaller the return value.
    ///
    /// ```rust
    /// // Initialize GalM Database instance.
    /// let galm: galm::Database = galm::Database::new();
    ///
    /// let sort_key = "王様";
    /// let mut vec = ["皇様", "玉様", "大様"];
    ///
    /// // Sort Example
    /// vec.sort_by_key( |candidate| galm.get_word_distance(sort_key, candidate) );
    ///
    /// assert_eq!(vec, ["玉様", "皇様", "大様"]);
    /// ```
    pub fn get_word_distance(&self, str1: &str, str2: &str) -> usize {

        // initialize table
        let table_x_size = str1.chars().count() + 1;
        let table_y_size = str2.chars().count() + 1;
        let mut table = vec![0; table_x_size * table_y_size];
        for i in 0..table_x_size {
            table[i] = i * self.max_distance_size;
        }
        for i in 0..table_y_size {
            table[i*(table_x_size)] = i * self.max_distance_size;
        }

        // テーブルを埋める
        // Extend of Levenshtein distance
        for i in 1..table_y_size {
            for j in 1..table_x_size {

                // 比較値の用意
                let up          = table[j+((i-1)*table_x_size)  ] + self.max_distance_size;
                let left        = table[j+(  i  *table_x_size)-1] + self.max_distance_size;
                let upper_left  = {
                    let char1 = str1.chars().nth(j-1).unwrap();
                    let char2 = str2.chars().nth(i-1).unwrap();
                    let c = if char1 == char2 { 0 } else { self.get_distance( &char1.to_string(), &char2.to_string() ) as usize };
                    table[j+((i-1)*table_x_size)-1] + c
                };

                // 最小値を求める
                table[j+(i*table_x_size)] = std::cmp::min(std::cmp::min(up, left), upper_left);
            }
        }

        // テーブルの右下（配列の最後）の値を返す
        return table[(table_x_size*table_y_size)-1 as usize];
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
