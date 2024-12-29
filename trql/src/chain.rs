#[derive(Clone)]
pub struct Chain<I: Iterator> {
    index: usize,
    iters: Vec<I>,
}

impl<I: Iterator> Chain<I> {
    pub fn new(iters: Vec<I>) -> Self {
        Self { index: 0, iters }
    }
}

impl<I: Iterator> Iterator for Chain<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.iters.get_mut(self.index) {
            if let Some(node) = iter.next() {
                Some(node)
            } else {
                self.index += 1;
                self.next()
            }
        } else {
            None
        }
    }
}
