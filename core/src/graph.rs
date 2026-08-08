use std::collections::{HashSet, HashMap, BTreeMap};
use std::sync::atomic::{AtomicUsize, Ordering};
use parking_lot::RwLock;

static ID_GEN: AtomicUsize = AtomicUsize::new(1);

pub struct Node {
    pub id: usize,
    pub depth: usize,
    pub val: f64,
    pub state: u8,
    pub evaluating: bool,
}

pub struct Graph {
    pub nodes: HashMap<usize, Node>,
    pub edges: HashMap<usize, HashSet<usize>>,
    pub rev: HashMap<usize, HashSet<usize>>,
    buckets: BTreeMap<usize, Vec<usize>>,
    pending: HashSet<usize>,
    batch: usize,
}

lazy_static::lazy_static! {
    pub static ref G: RwLock<Graph> = RwLock::new(Graph::new());
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            rev: HashMap::new(),
            buckets: BTreeMap::new(),
            pending: HashSet::new(),
            batch: 0,
        }
    }

    pub fn add(&mut self, val: f64) -> usize {
        let id = ID_GEN.fetch_add(1, Ordering::SeqCst);
        self.nodes.insert(id, Node { id, depth: 0, val, state: 0, evaluating: false });
        self.edges.insert(id, HashSet::new());
        self.rev.insert(id, HashSet::new());
        id
    }

    pub fn link(&mut self, src: usize, dst: usize) {
        if let Some(s) = self.edges.get_mut(&src) { s.insert(dst); }
        if let Some(r) = self.rev.get_mut(&dst) { r.insert(src); }
        
        let d = self.nodes[&src].depth + 1;
        if let Some(n) = self.nodes.get_mut(&dst) {
            n.depth = n.depth.max(d);
        }
    }

    fn push_q(&mut self, id: usize) {
        if self.pending.contains(&id) { return; }
        self.pending.insert(id);
        let depth = self.nodes.get(&id).map(|n| n.depth).unwrap_or(0);
        self.buckets.entry(depth).or_default().push(id);
    }

    fn pop_q(&mut self) -> Option<usize> {
        let first_key = *self.buckets.keys().next()?;
        let vec = self.buckets.get_mut(&first_key)?;
        let id = vec.remove(0);
        if vec.is_empty() {
            self.buckets.remove(&first_key);
        }
        self.pending.remove(&id);
        Some(id)
    }

    pub fn set(&mut self, id: usize, val: f64) {
        if let Some(n) = self.nodes.get_mut(&id) {
            if (n.val - val).abs() > f64::EPSILON {
                n.val = val;
                if let Some(edges) = self.edges.get(&id).cloned() {
                    for e in edges {
                        if let Some(target) = self.nodes.get_mut(&e) {
                            target.state = 2;
                        }
                        self.push_q(e);
                    }
                }
                self.propagate();
            }
        }
    }

    pub fn get(&mut self, id: usize) -> Result<f64, &'static str> {
        if let Some(n) = self.nodes.get(&id) {
            if n.evaluating {
                return Err("Cyclic dependency detected");
            }
            Ok(n.val)
        } else {
            Ok(0.0)
        }
    }

    pub fn batch_start(&mut self) { self.batch += 1; }
    
    pub fn batch_end(&mut self) {
        if self.batch > 0 { self.batch -= 1; }
        if self.batch == 0 { self.propagate(); }
    }

    fn propagate(&mut self) {
        if self.batch > 0 { return; }
        while let Some(curr) = self.pop_q() {
            if let Some(n) = self.nodes.get_mut(&curr) {
                n.state = 0;
            }
            if let Some(edges) = self.edges.get(&curr).cloned() {
                for e in edges {
                    if let Some(target) = self.nodes.get_mut(&e) {
                        target.state = 2;
                    }
                    self.push_q(e);
                }
            }
        }
    }
}