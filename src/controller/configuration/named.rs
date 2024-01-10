use std::borrow::Cow;

/// A list of named objects
pub(super) struct Named<T>(Vec<(Cow<'static, str>, T)>);

impl<T> Named<T> {
    /// Creates a new empty list
    pub(super) fn new() -> Self {
        Named(Vec::new())
    }

    /// Inserts an item to the list
    ///
    /// This does not check for repeated names
    pub(super) fn insert(&mut self, name: Cow<'static, str>, item: T) {
        self.0.push((name, item));
    }

    /// Does this list contain an object named `name`?
    pub(super) fn contains(&self, name: &str) -> bool {
        for (object_name, _) in self {
            if object_name == name {
                return true;
            }
        }

        false
    }
}

impl<'a, T> IntoIterator for &'a Named<T> {
    type Item = &'a (Cow<'static, str>, T);
    type IntoIter = std::slice::Iter<'a, (Cow<'static, str>, T)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
