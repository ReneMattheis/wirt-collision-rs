use std::ops;

#[derive(Clone, Copy)]
pub struct StackVec<T, const S: usize> {
    data: [T; S],
    len: usize,
}

impl<T, const S: usize> StackVec<T, S>
where
    T: Copy + Default,
{
    pub fn new() -> Self {
        Self {
            data: [T::default(); S],
            len: 0,
        }
    }

    pub fn push(&mut self, t: T) {
        self.data[self.len] = t;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut result = None;
        if !self.is_empty()
        {
            self.len -= 1;
            result = Some(self.data[self.len]);
        }
        result
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn capacity(&self) -> usize {
        S
    }

    pub fn clear(&mut self) {
        for index in 0..self.len() {
            self.data[index] = T::default();
        }
        self.len = 0;
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T, const S: usize> ops::Deref for StackVec<T, S> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.data[0..self.len]
    }
}

impl<T, const S: usize> ops::DerefMut for StackVec<T, S> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.data[0..self.len]
    }
}

impl<T, const S: usize> From<Vec<T>> for StackVec<T, S>
where
    T: Default + Copy,
{
    fn from(value: Vec<T>) -> Self {
        let mut stack_vec = Self::new();
        for v in value {
            stack_vec.push(v);
        }
        stack_vec
    }
}

impl<T, const S: usize> Extend<T> for StackVec<T, S>
where
    T: Default+Clone+Copy,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter {
            self.push(i);
        }
    }
}

impl<T, const S: usize> FromIterator<T> for StackVec<T, S>
where
    T: Default+Clone+Copy,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut result = StackVec::<T, S>::new();
        for i in iter {
            result.push(i);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_grow() {
        let mut stack_vec = StackVec::<u32, 4>::new();
        stack_vec.push(0);
        stack_vec.push(1);
        stack_vec.push(2);
        stack_vec.push(3);
        assert_eq!(4, stack_vec.len());
    }

    #[test]
    fn test_pop() {
        let mut stack_vec = StackVec::<u32, 4>::new();
        stack_vec.push(0);
        stack_vec.push(1);
        stack_vec.push(2);
        stack_vec.push(3);

        let mut count_pops = 0;
        while stack_vec.pop().is_some() {
            count_pops += 1;
        }
        assert_eq!(4, count_pops);
    }

    #[test]
    fn test_iter() {
        let mut stack_vec = StackVec::<u32, 2>::new();
        stack_vec.push(0);
        stack_vec.push(1);
        stack_vec.push(2);

        let mut count_iterations = 0;
        stack_vec.iter().for_each(|_| {
            count_iterations += 1;
        });
        assert_eq!(3, count_iterations);
    }
}
