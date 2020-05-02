//! search module for GalM.

fn get_levenshtein_distance(str1: &str, str2: &str) -> usize {

    // initialize galm
    let galm = Database::new();

    // initialize table
    let table_x_size = str1.chars().count() + 1;
    let table_y_size = str2.chars().count() + 1;
    let mut table = vec![0; table_x_size * table_y_size];
    for i in 0..table_x_size {
        table[i] = i * galm.max_distance_size;
    }
    for i in 0..table_y_size {
        table[i*(table_x_size)] = i * galm.max_distance_size;
    }

    // テーブルを埋める
    for i in 1..table_y_size {
        for j in 1..table_x_size {

            // 比較値の用意
            let up          = table[j+((i-1)*table_x_size)  ] + galm.max_distance_size;
            let left        = table[j+(  i  *table_x_size)-1] + galm.max_distance_size;
            let upper_left  = {
                let char1 = str1.chars().nth(j-1).unwrap();
                let char2 = str2.chars().nth(i-1).unwrap();
                let c = if char1 == char2 { 0 } else { galm.get_distance( &char1.to_string(), &char2.to_string() ) as usize };
                table[j+((i-1)*table_x_size)-1] + c
            };

            // 最小値を求める
            table[j+(i*table_x_size)] = std::cmp::min(std::cmp::min(up, left), upper_left);
        }
    }

    // テーブルの右下（配列の最後）の値を返す
    return table[(table_x_size*table_y_size)-1 as usize];
}

use crate::Database;

/// Extend Iterator for search similar word.
pub trait Iterator<'a, 'b> {
    type Item;
    fn get_similar_word(self, input_param: &'b str) -> &'a str;
}


/// Extend Iterator for search similar word.
impl<'a, 'b> Iterator<'a, 'b> for std::slice::Iter<'a, &str>
{
    type Item = &'a str;

    /// Get similar word.
    /// 
    /// ```
    /// use galm::search::Iterator;
    /// 
    /// let result = [
    ///     "-h", "--help",
    ///     "-v", "--version"
    /// ].iter().get_similar_word("nelp");
    /// 
    /// assert_eq!(result, "--help");
    /// ```
    fn get_similar_word(self, input_param: &'b str) -> &'a str {
        self.fold(("", usize::max_value()), |most_similar_param, param| {
            let distance = get_levenshtein_distance(input_param, param);
            if distance < most_similar_param.1 { (param, distance) } else { most_similar_param }
        }).0
    }
}
