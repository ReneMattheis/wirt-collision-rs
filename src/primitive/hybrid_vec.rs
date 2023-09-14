use std::ops;

use super::StackVec;

#[derive(Clone)]
enum VecType<T, const S: usize> {
    Stack(StackVec<T, S>),
    Heap(Vec<T>),
}

impl<T, const S: usize> Default for VecType<T, S>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self::Stack(StackVec::<T, S>::new())
    }
}

#[derive(Default, Clone)]
pub struct HybridVec<T: Copy+Clone, const S: usize> {
    vec: VecType<T, S>,
}

impl<T, const S: usize> HybridVec<T, S>
where
    T: Copy + Default,
{
    pub fn new() -> Self {
        Self {
            vec: VecType::Stack(StackVec::new()),
        }
    }

    pub fn push(&mut self, t: T) {
        let mut transmutation_necessary = false;
        match &mut self.vec {
            VecType::Stack(sv) => {
                if sv.len() == sv.capacity() {
                    transmutation_necessary = true;
                } else {
                    sv.push(t);
                }
            }
            VecType::Heap(hv) => hv.push(t),
        }

        if transmutation_necessary {
            let mut hv = Vec::<T>::with_capacity(self.capacity() * 2);
            if let VecType::Stack(sv) = &mut self.vec {
                hv.extend_from_slice(sv);
            }
            hv.push(t);
            self.vec = VecType::Heap(hv);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match &mut self.vec {
            VecType::Stack(sv) => sv.pop(),
            VecType::Heap(hv) => hv.pop(),
        }
    }

    pub fn len(&self) -> usize {
        match &self.vec {
            VecType::Stack(sv) => sv.len(),
            VecType::Heap(hv) => hv.len(),
        }
    }

    pub fn capacity(&self) -> usize {
        match &self.vec {
            VecType::Stack(sv) => sv.capacity(),
            VecType::Heap(hv) => hv.capacity(),
        }
    }

    pub fn clear(&mut self) {
        match &mut self.vec {
            VecType::Stack(sv) => sv.clear(),
            VecType::Heap(hv) => hv.clear(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.vec {
            VecType::Stack(sv) => sv.is_empty(),
            VecType::Heap(hv) => hv.is_empty(),
        }
    }
}

impl<T: Copy, const S: usize> ops::Deref for HybridVec<T, S> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        match &self.vec {
            VecType::Stack(sv) => sv.deref(),
            VecType::Heap(hv) => hv.deref(),
        }
    }
}

impl<T: Copy, const S: usize> ops::DerefMut for HybridVec<T, S> {
    fn deref_mut(&mut self) -> &mut [T] {
        match &mut self.vec {
            VecType::Stack(sv) => sv.deref_mut(),
            VecType::Heap(hv) => hv.deref_mut(),
        }
    }
}

impl<T, const S: usize> From<Vec<T>> for HybridVec<T, S>
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

impl<T, const S: usize> Extend<T> for HybridVec<T, S>
where
    T: Default + Clone + Copy,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter {
            self.push(i);
        }
    }
}

impl<T, const S: usize> FromIterator<T> for HybridVec<T, S>
where
    T: Default + Clone + Copy,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut result = HybridVec::<T, S>::new();
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
        let mut stack_vec = HybridVec::<u32, 2>::new();
        stack_vec.push(0);
        stack_vec.push(1);
        stack_vec.push(2);
    }

    #[test]
    fn test_slice() {
        let mut stack_vec = HybridVec::<u32, 2>::new();
        stack_vec.push(0);
        stack_vec.push(1);
        stack_vec.push(2);

        let mut count_iterations = 0;
        for _ in stack_vec.iter() {
            count_iterations += 1;
        }
        assert_eq!(3, count_iterations);
    }
}
