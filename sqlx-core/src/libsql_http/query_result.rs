use std::iter::{Extend, IntoIterator};

#[derive(Debug, Default)]
pub struct LibsqlHttpQueryResult {
}

impl Extend<LibsqlHttpQueryResult> for LibsqlHttpQueryResult {
    fn extend<T: IntoIterator<Item = LibsqlHttpQueryResult>>(&mut self, iter: T) {
    }
}
