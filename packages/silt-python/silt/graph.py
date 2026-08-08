import weakref

class CyclicDependencyError(Exception):
    pass

class PriorityBucketQueue:
    def __init__(self):
        self.buckets = {}
        self.pending = set()
        self.min_d = float('inf')
        self.max_d = -1

    def push(self, node):
        if node in self.pending:
            return
        self.pending.add(node)
        d = node.depth
        if d not in self.buckets:
            self.buckets[d] = []
        self.buckets[d].append(node)
        if d < self.min_d: self.min_d = d
        if d > self.max_d: self.max_d = d

    def pop(self):
        while self.min_d <= self.max_d:
            b = self.buckets.get(self.min_d)
            if b:
                node = b.pop(0)
                self.pending.remove(node)
                if not b:
                    del self.buckets[self.min_d]
                    self.min_d += 1
                return node
            self.min_d += 1
        self.min_d = float('inf')
        self.max_d = -1
        return None

class Node:
    _id = 0
    def __init__(self):
        self.id = Node._id
        Node._id += 1
        self.obs = weakref.WeakSet()
        self.src = set()
        self.depth = 0
        self.state = 0
        self.evaluating = False

class Engine:
    def __init__(self):
        self.active = None
        self.batch = 0
        self.bq = PriorityBucketQueue()

eng = Engine()

def propagate():
    if eng.batch > 0:
        return
    while True:
        node = eng.bq.pop()
        if not node:
            break
        if node.state == 2:
            node.update()
        elif node.state == 1:
            for s in node.src:
                if s.state > 0:
                    s.update()
            if node.state == 2:
                node.update()
        node.state = 0

def link(n):
    if eng.active:
        if n not in eng.active.src:
            eng.active.src.add(n)
            n.obs.add(eng.active)
            eng.active.depth = max(eng.active.depth, n.depth + 1)

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
            for o in list(self.obs):
                o.state = 2
                eng.bq.push(o)
            propagate()

class Computed(Node):
    def __init__(self, fn):
        super().__init__()
        self.fn = fn
        self.val = None
        self.error = None
        self.dirty = True

    def update(self):
        if self.evaluating:
            raise CyclicDependencyError(f"Cycle detected at node {self.id}")
        self.state = 0
        old = self.val
        for s in self.src:
            s.obs.discard(self)
        self.src.clear()

        prev = eng.active
        eng.active = self
        self.evaluating = True
        try:
            self.val = self.fn()
            self.error = None
        except Exception as e:
            self.error = e
        finally:
            self.evaluating = False
            eng.active = prev

        if old != self.val or self.dirty:
            self.dirty = False
            for o in list(self.obs):
                o.state = 2
                eng.bq.push(o)

    def get(self):
        if self.dirty or self.state > 0:
            self.update()
        if self.error:
            raise self.error
        link(self)
        return self.val