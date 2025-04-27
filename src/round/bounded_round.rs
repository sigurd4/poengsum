use core::cmp::Ordering;

#[derive(Clone, Copy)]
pub enum BoundedRound
{
    One(usize),
    Range {
        start: usize,
        end: usize
    }
}

impl BoundedRound
{
    pub fn new(start: usize, end: Option<usize>) -> Self
    {
        if let Some(end) = end && start != end
        {
            return Self::Range { start, end }
        }
        Self::One(start)
    }

    pub fn index<T>(self, slice: &[T]) -> Option<&[T]>
    {
        match self
        {
            BoundedRound::One(runde) => slice.get(runde).map(core::slice::from_ref),
            BoundedRound::Range { start, end } => slice.get(start..=end)
        }
    }

    pub fn undo(self) -> Option<Self>
    {
        match self
        {
            Self::One(_) => None,
            Self::Range { start, mut end } => {
                match end.cmp(&start)
                {
                    Ordering::Less => end += 1,
                    Ordering::Equal => return None,
                    Ordering::Greater => end -= 1,
                }
                Some(Self::new(start, Some(end)))
            }
        }
    }
}