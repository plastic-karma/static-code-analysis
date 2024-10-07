#[cfg(test)]
mod tests {
    use crate::collections::push_back_iterator::PushBackIterator;

    #[test]
    fn test_regular_iterator() {
        let data = vec![1, 2, 3, 4, 5];
        let mut iter = PushBackIterator::new(data.iter());
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_with_push_back() {
        let data = vec![1, 2, 3, 4, 5];
        let mut iter = PushBackIterator::new(data.iter());
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        iter.push_back(&8);
        iter.push_back(&9);
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_peek() {
        let data = vec![1, 2, 3, 4, 5];
        let mut iter = PushBackIterator::new(data.iter());
        assert_eq!(iter.peek(), Some(&&1));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.peek(), Some(&&2));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.peek(), Some(&&3));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.peek(), Some(&&4));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.peek(), Some(&&5));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.peek(), None);
        assert_eq!(iter.next(), None);
    }
}