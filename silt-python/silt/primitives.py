from .engine import Node, eng, link, propagate, emit

class Signal(Node):
    def __init__(self, val):
        super().__init__()
        self.val = val

    def get(self):
        link(self)
        return self.val

    def set(self, val):
        if self.val != val:
            self.val = val
            emit('set', self.id, val)
            # weakset iteration needs list copy
            for o in list(self.obs):
                o.state = 2
                if o not in eng.q:
                    eng.q.append(o)
            propagate()

class Computed(Node):
    def __init__(self, fn):
        super().__init__()
        self.fn = fn
        self.val = None
        self.dirty = True
        
    def update(self):
        self.state = 0
        old = self.val
        for s in self.src:
            s.obs.discard(self)
        self.src.clear()
        
        prev = eng.active
        eng.active = self
        try:
            new_val = self.fn()
        finally:
            eng.active = prev
            
        if old != new_val or self.dirty:
            self.val = new_val
            self.dirty = False
            for o in list(self.obs):
                o.state = 2
                if o not in eng.q:
                    eng.q.append(o)

    def get(self):
        if self.dirty or self.state > 0:
            self.update()
        link(self)
        return self.val

class Effect(Node):
    def __init__(self, fn):
        super().__init__()
        self.fn = fn
        eng.roots.add(self)
        self.update()

    def update(self):
        self.state = 0
        for s in self.src:
            s.obs.discard(self)
        self.src.clear()
        
        prev = eng.active
        eng.active = self
        try:
            self.fn()
        finally:
            eng.active = prev
            
    def stop(self):
        eng.roots.discard(self)
        for s in self.src:
            s.obs.discard(self)
        self.src.clear()