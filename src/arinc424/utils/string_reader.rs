pub struct StringReader<'a> {
    data: &'a str,
    pointer: usize,
}

impl<'a> StringReader<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data, pointer: 0 }
    }

    pub fn read(&mut self, count: usize) -> Option<&str> {
        let slice = self.data.get(self.pointer..(self.pointer + count));
        return slice.and_then(|slice| {
            self.pointer += count;
            Some(slice)
        });
    }

    pub fn skip(&mut self, count: usize) -> &mut Self {
        self.pointer += count;
        self
    }

    pub fn peek(&self, count: usize) -> Option<&str> {
        self.data.get(self.pointer..(self.pointer + count))
    }

    pub fn peek_after(&self, skip: usize, count: usize) -> Option<&str> {
        self.data
            .get((self.pointer + skip)..(self.pointer + skip + count))
    }
}
