#![doc = include_str!("../README.md")]

// #[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[derive(Debug)]
pub struct Characters {
    pub names: Vec<&'static str>,
    pub costs: Vec<usize>,
}


macro_rules! load_csv {
    ($path:tt) => {
        {
            let mut characters: Vec<&str> = Vec::new();
            let costs: Vec<usize> = include_str!(
                $path
            ).lines().enumerate().filter_map( |(line_index, line_value)| {
                if line_index != 0 {
                    Some(
                        line_value.split(',').map( |comma_value| {
                            comma_value.parse::<usize>().unwrap()
                        }).collect::<Vec<usize>>()
                    )
                } else {
                    characters = line_value.split(',').collect();
                    None
                }
            }).into_iter().flatten().collect();

            Characters { names: characters, costs: costs }
        }
    };
}


/// Database
#[derive(Debug)]
pub struct Database {
    pub characters: Characters,
    pub max_distance_size: usize
}

impl Default for Database {
    /// Initialize GalM Database instance.
    ///
    /// ```rust
    /// let galm: galm::Database = galm::Database::default();
    /// ```
    fn default() -> Self {
        Self {
            characters: load_csv!("./../docs/assets/csv/characters.csv"),
            max_distance_size: 255
        }
    }
}

/// Database
impl Database {
    /// Get the matching rate of character.
    /// Range is 0..[`galm::Database.max_distance_size`](#structfield.max_distance_size).
    /// The more similar the two arguments, the smaller the return value.
    ///
    /// ```rust
    /// // Initialize GalM Database instance.
    /// let galm: galm::Database = galm::Database::default();
    ///
    /// // Get the matching rate of character.
    /// let distance: u8 = galm.get_distance("王", "玉");
    ///
    /// assert_eq!(distance, 22);
    /// ```
    pub fn get_distance(&self, from: &str, to: &str) -> u8 {
        let index_x = match self.characters.names.iter().position(|&r| r == from) {
            Some(i) => i,
            None => return self.max_distance_size  as u8
        };
        let index_y = match self.characters.names.iter().position(|&r| r == to) {
            Some(i) => i,
            None => return self.max_distance_size  as u8
        };
        match self.characters.costs[index_y + ( index_x * self.characters.names.len() )] {
            i if self.max_distance_size < i => { self.max_distance_size as u8 },
            i => i as u8,
        }
    }

    /// Get the matching rate of word.
    /// Range is 0..[`std::usize::MAX`](https://doc.rust-lang.org/std/usize/constant.MAX.html).
    /// The more similar the two arguments, the smaller the return value.
    ///
    /// ```rust
    /// // Initialize GalM Database instance.
    /// let galm: galm::Database = galm::Database::default();
    ///
    /// let sort_key = "王様";
    /// let mut vec = ["皇様", "玉様", "大様"];
    ///
    /// // Sort Example
    /// vec.sort_by_key( |candidate| galm.get_word_distance(sort_key, candidate) );
    ///
    /// assert_eq!(vec, ["玉様", "大様", "皇様"]);
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
        table[(table_x_size*table_y_size) - 1]
    }
}


/// Initialize GalM Database instance.
///
/// ```rust
/// let galm: galm::Database = galm::new();
/// ```
pub fn new() -> Database {
    Database::default()
}

pub mod search;
