use std::collections::VecDeque;
use std::collections::HashSet;
use std::hash::Hash;

use persistent_rope::*;

struct History<T, M> {
    buffers: VecDeque<Rope<T, M>>,
    size: usize,
}

pub struct Buffer<T, M> {
    history: History<T, M>,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum Marker {
    Linebreak,
}

pub use self::Marker::*;

impl<T, M> History<T, M> {

    fn latest(&self) -> &Rope<T, M> {
        match self.buffers.back() {
            Some(rope) => rope,
            None => panic!("history must contain at least one item"),
        }
    }

    fn read<F>(&self, mut operation: F)
        where F: FnMut(&Rope<T, M>) {

        operation(self.latest());
    }

    fn write<F>(&mut self, mut operation: F)
        where F: FnMut(&Rope<T, M>) -> Rope<T, M> {

        let new = operation(self.latest());

        if self.buffers.len() >= self.size {
            self.buffers.pop_back();
        }

        self.buffers.push_front(new);
    }
}

impl<T: Clone, M: Eq + Hash + Copy> Buffer<T, M> {

    pub fn new(initial: Rope<T, M>, history_size: usize) -> Self {
        if history_size < 1 {
            panic!("history size must be at least 1");
        }

        let mut history = VecDeque::new();
        history.push_front(initial);

        Buffer {
            history: History {
                buffers: history,
                size: history_size,
            }
        }
    }

    
    pub fn generic_load<F, E>(chunk_size: usize,
                                history_size: usize,
                                mut loader: F) -> Result<Self, E>
        where F: FnMut(Chunk<T, M>) -> Result<Option<Chunk<T, M>>, E> {

        match Rope::generic_load(chunk_size, loader) {
            Err(e) => Err(e),
            Ok(rope) => Ok(Self::new(rope, history_size)),
        }
    }

    pub fn len(&self) -> usize {
        self.history.latest().len()
    }

    pub fn marker_count(&self, marker: M) -> usize {
        self.history.latest().marker_count(marker)
    }

    pub fn read<F>(&self, mut operation: F)
        where F: FnMut(&Rope<T, M>) {

        self.history.read(operation);
    }

    pub fn write<F>(&mut self, mut operation: F)
        where F: FnMut(&Rope<T, M>) -> Rope<T, M> {

        self.history.write(operation);
    }

}

impl<'a, T: Clone, M: Eq + Hash + Copy> IntoIterator for &'a Buffer<T, M> {

    type Item = &'a T;
    type IntoIter = Values<'a, T, M>;

    fn into_iter(self) -> Self::IntoIter {
        self.history.latest().into_iter()
    }
}
