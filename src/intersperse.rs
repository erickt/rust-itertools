#[derive(Clone)]
/// An iterator adaptor to insert a particular value
/// between each element of the adapted iterator.
///
/// Iterator element type is **I::Item**
pub struct Intersperse<I> where
    I: Iterator,
{
    element: I::Item,
    iter: I,
    peek: Option<I::Item>,
}

impl<I> Intersperse<I> where
    I: Iterator,
{
    /// Create a new Intersperse iterator
    pub fn new(mut iter: I, elt: I::Item) -> Self
    {
        Intersperse{peek: iter.next(), iter: iter, element: elt}
    }
}

impl<I> Iterator for Intersperse<I> where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<I::Item>
    {
        if self.peek.is_some() {
            self.peek.take()
        } else {
            self.peek = self.iter.next();
            if self.peek.is_some() {
                Some(self.element.clone())
            } else {
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let (mut low, mut hi) = self.iter.size_hint();
        if low > 0 {
            low = low.saturating_add((low - 1));
        }
        hi = hi.and_then(|x| if x > 0 {
            x.checked_add(x - 1)
        } else { Some (x) });
        (low, hi)
    }
}
