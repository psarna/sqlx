use std::iter::{Extend, IntoIterator};

#[derive(Debug, Default)]
pub struct LibsqlClientQueryResult {}

impl Extend<LibsqlClientQueryResult> for LibsqlClientQueryResult {
    fn extend<T: IntoIterator<Item = LibsqlClientQueryResult>>(&mut self, iter: T) {}
}
