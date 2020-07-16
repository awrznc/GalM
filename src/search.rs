//! search module for GalM.

use crate::Database;

/// Extend Iterator for search similar word.
pub trait Iterator<'a, 'b> {
    type Item;
    fn get_similar_word(self, input_param: &'b str) -> &'a str;
    fn partial_match(self, input_param: &'b str) -> Vec<&'a str>;
}

/// Extend Iterator for search similar word.
impl<'a, 'b> Iterator<'a, 'b> for std::slice::Iter<'a, &str> {

    type Item = &'a str;

    /// Get similar word.
    ///
    /// ```
    /// use galm::search::Iterator;
    ///
    /// // "danana" => "banana"
    /// let result = [
    ///     "banana",
    ///     "nana",
    ///     "nabana"
    /// ].iter().get_similar_word("danana");
    ///
    /// assert_eq!("banana", result);
    /// ```
    fn get_similar_word(self, input_param: &'b str) -> &'a str {
        let galm = Database::new();
        self.fold(("", usize::max_value()), |most_similar_param, param| {
            let distance = galm.get_word_distance(input_param, param);
            if distance < most_similar_param.1 { (param, distance) } else { most_similar_param }
        }).0
    }


    /// Partial match and sort.
    ///
    /// ```
    /// use galm::search::Iterator;
    ///
    /// let result = [
    ///     "banana",
    ///     "nana",
    ///     "nabana"
    /// ].iter().partial_match("nana");
    ///
    /// assert_eq!(vec!["nana", "banana"], result);
    /// ```
    fn partial_match(self, matching_string: &'b str) -> Vec<&'a str> {
        let mut result: Vec<&'a str> = self.filter(|string| {
            string.contains(matching_string)
        }).cloned().collect();
        result.sort_by_key( |string| string.chars().count() );
        result
    }
}
