//! An implementation of a text editing buffer, Uses `persistent_rope` to
//! store the actual data, and adds functionality related to loading,
//! saving, querying, and editing.

use std::collections::VecDeque;
use std::hash::Hash;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use persistent_rope::{Rope, Values};
pub use persistent_rope::Chunk;

const DEFAULT_HISTORY_SIZE: usize = 1;
const CHUNK_SIZE: usize = 512;

struct History<T, M> {
    buffers: VecDeque<Rope<T, M>>,
    size: usize,
}

pub struct Buffer<T, M> {
    file_name: Option<PathBuf>,
    history: History<T, M>,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum Marker {
    Linebreak,
}

pub use self::Marker::*;

pub trait GetLine {
    fn get_line(&self, line: usize) -> Option<Box<String>>;
}

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
            file_name: None,
            history: History {
                buffers: history,
                size: DEFAULT_HISTORY_SIZE,
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

/*
pub trait GetLine {
    fn get_line(&self, line: usize) -> Option<Box<String>>;
}
*/

impl GetLine for Buffer<u8, Marker> {

    fn get_line(&self, line: usize) -> Option<Box<String>> {
        unimplemented!()
    }
}

impl Buffer<u8, Marker> {

    pub fn from_bytes<R: Read>(mut bytes: R) -> io::Result<Self> {
        let mut buffer: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

        Buffer::from_chunks(|| {
            let result = bytes.read(&mut buffer);
                
            match result {
                Err(e) => Err(e),
                Ok(0) => Ok(None),

                Ok(bytes_read) => {
                    let mut chunk = Chunk::with_capacity(CHUNK_SIZE);
                    chunk.extend_from_slice(&buffer[0..bytes_read]);
                    
                    for (i, &byte) in buffer[0..bytes_read].iter().enumerate() {
                        if byte == 10 {
                            chunk.mark_at(Linebreak, i);
                        }
                    }

                    Ok(Some(chunk))
                }
            }
        })
    }
}

impl<'a, T: Clone, M: Eq + Hash + Copy> IntoIterator for &'a Buffer<T, M> {

    type Item = &'a T;
    type IntoIter = Values<'a, T, M>;

    fn into_iter(self) -> Self::IntoIter {
        self.history.latest().into_iter()
    }
}
