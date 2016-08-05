extern crate rustbox;
extern crate sphinx;

use std::collections::BTreeMap;
use std::default::Default;

pub use rustbox::RustBox;
pub use rustbox::Event;
pub use rustbox::keyboard::Key;
pub use rustbox::Style;
pub use rustbox::Color;

use sphinx::buffer::*;

type PaneID = usize;

enum Pane<T, M> {
    Loaded {
        buffer: Buffer<T, M>,
        position: (usize, usize),
    },
    Unloaded,
}

use self::Pane::*;

enum SplitType {
    Vertical,
    Horizontal,
}

use self::SplitType::*;

enum Layout {
    Leaf(PaneID),
    Split(SplitType, Vec<Layout>),
}

use self::Layout::*;

pub struct SphinxBox<T, M> {
    pub rustbox: RustBox,
    panes: BTreeMap<PaneID, Pane<T, M>>,
    active_pane: PaneID,
    layout: Layout,
}

pub enum Action {
    DoNothing,
    ClosePane,
    Exit,
}

pub use self::Action::*;

impl<T, M> SphinxBox<T, M>
    where Buffer<T, M>: GetLine {

    pub fn new() -> Result<Self, rustbox::InitError> {
        let mut panes = BTreeMap::new();
        panes.insert(1, Unloaded);

        match RustBox::init(Default::default()) {
            Err(e) => Err(e),
            Ok(rustbox) => Ok(SphinxBox {
                rustbox: rustbox,
                panes: panes,
                active_pane: 1,
                layout: Leaf(1),
            })
        }
    }

    pub fn eventloop<F>(mut self, mut body: F)
        where F: FnMut(&mut Self, rustbox::Event) -> Action {

        loop {
            let event = match self.rustbox.poll_event(false) {
                Ok(event) => event,
                Err(e) => {
                    std::mem::drop(self);
                    println!("{:?}", e);
                    break;
                }
            };

            match body(&mut self, event) {
                Exit => break,
                DoNothing => (),
                _ => unimplemented!(),
            }
        }
    }
}
