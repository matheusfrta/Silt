use crate::engine::{ENG, link, propagate};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Signal<T> {
    pub id: usize,
    val: Rc<RefCell<T>>,
}

impl<T: Clone + PartialEq + 'static> Signal<T> {
    pub fn new(val: T) -> Self {
        let id = ENG.with(|e| e.borrow_mut().add());
        Self { id, val: Rc::new(RefCell::new(val)) }
    }

    pub fn get(&self) -> T {
        link(self.id);
        self.val.borrow().clone()
    }

    pub fn set(&self, v: T) {
        let mut changed = false;
        if *self.val.borrow() != v {
            *self.val.borrow_mut() = v;
            changed = true;
        }
        if changed {
            ENG.with(|e| {
                let mut e = e.borrow_mut();
                let obs = e.nodes[self.id].obs.clone();
                for o in obs {
                    e.nodes[o].state = 2;
                    if !e.q.contains(&o) {
                        e.q.push(o);
                    }
                }
            });
            propagate();
        }
    }
}

pub struct Computed<T> {
    pub id: usize,
    val: Rc<RefCell<Option<T>>>,
}

impl<T: Clone + PartialEq + 'static> Computed<T> {
    pub fn new<F: FnMut() -> T + 'static>(mut f: F) -> Self {
        let id = ENG.with(|e| e.borrow_mut().add());
        let val = Rc::new(RefCell::new(None));
        let val_clone = val.clone();

        let cb = Rc::new(RefCell::new(move || {
            ENG.with(|e| {
                let mut e = e.borrow_mut();
                let srcs = e.nodes[id].src.clone();
                for s in srcs {
                    e.nodes[s].obs.remove(&id);
                }
                e.nodes[id].src.clear();
                e.active = Some(id);
            });
            let new_val = f();
            *val_clone.borrow_mut() = Some(new_val);
            ENG.with(|e| {
                let mut e = e.borrow_mut();
                e.active = None;
                let obs = e.nodes[id].obs.clone();
                for o in obs {
                    e.nodes[o].state = 2;
                    if !e.q.contains(&o) { e.q.push(o); }
                }
            });
        }));

        ENG.with(|e| e.borrow_mut().nodes[id].cb = Some(cb.clone()));
        cb.borrow_mut()();
        Self { id, val }
    }

    pub fn get(&self) -> T {
        link(self.id);
        self.val.borrow().as_ref().unwrap().clone()
    }
}

pub struct Effect {
    pub id: usize,
}

impl Effect {
    pub fn new<F: FnMut() + 'static>(mut f: F) -> Self {
        let id = ENG.with(|e| e.borrow_mut().add());
        
        let cb = Rc::new(RefCell::new(move || {
            ENG.with(|e| {
                let mut e = e.borrow_mut();
                let srcs = e.nodes[id].src.clone();
                for s in srcs {
                    e.nodes[s].obs.remove(&id);
                }
                e.nodes[id].src.clear();
                e.active = Some(id);
            });
            f();
            ENG.with(|e| e.borrow_mut().active = None);
        }));

        ENG.with(|e| {
            let mut e = e.borrow_mut();
            e.nodes[id].cb = Some(cb.clone());
            e.roots.insert(id);
        });
        
        cb.borrow_mut()();
        Self { id }
    }

    pub fn stop(&self) {
        ENG.with(|e| {
            let mut e = e.borrow_mut();
            e.roots.remove(&self.id);
            let srcs = e.nodes[self.id].src.clone();
            for s in srcs {
                e.nodes[s].obs.remove(&self.id);
            }
            e.nodes[self.id].src.clear();
        });
    }
}