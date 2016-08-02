//! An implementation of a text editing buffer, Uses `persistent_rope` to
//! store the actual data, and adds functionality related to loading,
//! saving, querying, and editing.

use std::collections::VecDeque;
use std::collections::HashSet;
use std::hash::Hash;

use persistent_rope::{Rope, Values};
pub use persistent_rope::Chunk;

const default_history_size: usize = 1;

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

    pub fn new(initial: Rope<T, M>) -> Self {
        let mut history = VecDeque::new();
        history.push_front(initial);

        Buffer {
            history: History {
                buffers: history,
                size: default_history_size,
            }
        }
    }

    /// Since `Buffer` uses `persistent_rope` to store data, we must consider
    /// the nature of the rope data structure if we want to be as efficient as
    /// possible. See `Rope::from_chunks` for details on how to use this.
    pub fn from_chunks<F, E>(mut loader: F) -> Result<Self, E>
        where F: FnMut() -> Result<Option<Chunk<T, M>>, E> {

        match Rope::from_chunks(loader) {
            Err(e) => Err(e),
            Ok(rope) => Ok(Self::new(rope)),
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
