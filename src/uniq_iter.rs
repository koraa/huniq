struct UniqueIter {}

impl Iterator<Item = &[u8]> for UniqueIter {
    type Item = &[u8];

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

trait UniqueIterator: Iterator {}

impl UniqueIterator {
    fn unique(self) {}
}
