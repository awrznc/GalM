//! search module for GalM.

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
        let galm = Database::new();
        self.fold(("", usize::max_value()), |most_similar_param, param| {
            let distance = galm.get_word_distance(input_param, param);
            if distance < most_similar_param.1 { (param, distance) } else { most_similar_param }
        }).0
    }
}
