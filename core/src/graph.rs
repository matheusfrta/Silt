use std::collections::{HashSet, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};
use parking_lot::RwLock;

static ID_GEN: AtomicUsize = AtomicUsize::new(1);

pub struct Node {
    pub id: usize,
    pub depth: usize,
    pub val: f64,
}

pub struct Graph {
    pub nodes: HashMap<usize, Node>,
    pub edges: HashMap<usize, HashSet<usize>>,
    pub rev: HashMap<usize, HashSet<usize>>,
    q: Vec<usize>,
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
            q: vec![],
            batch: 0,
        }
    }

    pub fn add(&mut self, val: f64) -> usize {
        let id = ID_GEN.fetch_add(1, Ordering::SeqCst);
        self.nodes.insert(id, Node { id, depth: 0, val });
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

    pub fn set(&mut self, id: usize, val: f64) {
        if let Some(n) = self.nodes.get_mut(&id) {
            if (n.val - val).abs() > f64::EPSILON {
                n.val = val;
                if let Some(edges) = self.edges.get(&id) {
                    for &e in edges {
                        if !self.q.contains(&e) { self.q.push(e); }
                    }
                }
                self.propagate();
            }
        }
    }

    pub fn get(&self, id: usize) -> f64 {
        self.nodes.get(&id).map(|n| n.val).unwrap_or(0.0)
    }

    pub fn batch_start(&mut self) { self.batch += 1; }
    
    pub fn batch_end(&mut self) {
        if self.batch > 0 { self.batch -= 1; }
        if self.batch == 0 { self.propagate(); }
    }

    fn propagate(&mut self) {
        if self.batch > 0 { return; }
        while !self.q.is_empty() {
            self.q.sort_by_key(|id| self.nodes.get(id).map(|n| n.depth).unwrap_or(0));
            let curr = self.q.remove(0);
            
            if let Some(edges) = self.edges.get(&curr).cloned() {
                for e in edges {
                    if !self.q.contains(&e) { self.q.push(e); }
                }
            }
        }
    }
}