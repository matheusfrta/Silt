use std::collections::HashSet;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

thread_local! {
    pub static ENG: RefCell<Engine> = RefCell::new(Engine::new());
}

pub type Cb = Rc<RefCell<dyn FnMut()>>;

pub struct Node {
    pub id: usize,
    pub obs: HashSet<usize>,
    pub src: HashSet<usize>,
    pub depth: usize,
    pub state: u8,
    pub cb: Option<Cb>,
    pub weak: bool,
}

pub struct Engine {
    pub nodes: Vec<Node>,
    pub active: Option<usize>,
    pub batch: usize,
    pub q: Vec<usize>,
    pub roots: HashSet<usize>,
}

impl Engine {
    pub fn new() -> Self {
        Self { nodes: vec![], active: None, batch: 0, q: vec![], roots: HashSet::new() }
    }

    pub fn add(&mut self) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node { 
            id, 
            obs: HashSet::new(), 
            src: HashSet::new(), 
            depth: 0, 
            state: 0, 
            cb: None,
            weak: false,
        });
        id
    }
}

pub fn link(id: usize) {
    ENG.with(|e| {
        let mut e = e.borrow_mut();
        if let Some(act) = e.active {
            if !e.nodes[act].src.contains(&id) {
                e.nodes[act].src.insert(id);
                e.nodes[id].obs.insert(act);
                let d = e.nodes[id].depth;
                e.nodes[act].depth = e.nodes[act].depth.max(d + 1);
            }
        }
    });
}

pub fn propagate() {
    ENG.with(|e| {
        let mut e = e.borrow_mut();
        if e.batch > 0 { return; }
        e.q.sort_by_key(|&id| e.nodes[id].depth);

        while !e.q.is_empty() {
            let n = e.q.remove(0);
            let state = e.nodes[n].state;
            if state == 2 {
                if let Some(cb) = e.nodes[n].cb.clone() {
                    drop(e);
                    cb.borrow_mut()();
                    e = ENG.with(|ex| ex.borrow_mut());
                }
            }
            e.nodes[n].state = 0;
        }
    });
}