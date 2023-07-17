#[derive(Clone)]
pub struct TodoList<T> {
    list: Vec<T>,
    processed_counter: usize,
}

impl<T> Default for TodoList<T> {
    fn default() -> Self {
        Self {
            list: vec![],
            processed_counter: 0,
        }
    }
}

impl<T> TodoList<T> {
    pub fn get_new(&mut self) -> &[T] {
        let start = self.processed_counter;
        self.processed_counter = self.list.len();
        &self.list[start..self.processed_counter]
    }

    pub fn get_all(&self) -> &[T] {
        &self.list
    }

    pub fn push(&mut self, entry: T) {
        self.list.push(entry);
    }

    pub fn contains(&self, entry: &T) -> bool
    where
        T: PartialEq,
    {
        self.list.contains(entry)
    }
}
