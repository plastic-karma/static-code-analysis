use std::iter::Peekable;

#[cfg(test)]
mod tests;

/// An iterator that allows pushing back elements
pub struct PushBackIterator<I>
where
    I: Iterator,
{
    iter: Peekable<I>,
    pushed_back: Vec<I::Item>,
}

impl<I> PushBackIterator<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        PushBackIterator {
            iter: iter.peekable(),
            pushed_back: Vec::new(),
        }
    }

    pub fn push_back(&mut self, item: I::Item) {
        self.pushed_back.push(item);
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        if self.pushed_back.is_empty() {
            self.iter.peek()
        } else {
            Some(self.pushed_back.last().unwrap())
        }
    }
}

impl<I> Iterator for PushBackIterator<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.pushed_back.pop() {
            Some(item)
        } else {
            self.iter.next()
        }
    }
}