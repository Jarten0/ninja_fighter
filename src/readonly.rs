pub struct ReadOnly<T>(T);

impl<T> ReadOnly<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn get(&self) -> &T {
        &self.0
    }
}
