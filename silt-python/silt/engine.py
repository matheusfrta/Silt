class Node:
    _id = 0
    def __init__(self):
        self.id = Node._id
        Node._id += 1
        self.obs = set()
        self.src = set()
        self.depth = 0
        self.state = 0
    
    def update(self):
        pass

class Engine:
    def __init__(self):
        self.active = None
        self.batch = 0
        self.q = []

eng = Engine()

def propagate():
    if eng.batch > 0: 
        return
    eng.q.sort(key=lambda x: x.depth)
    while eng.q:
        n = eng.q.pop(0)
        if n.state == 2:
            n.update()
        elif n.state == 1:
            for s in n.src:
                if s.state > 0:
                    s.update()
            if n.state == 2:
                n.update()
        n.state = 0

def link(n):
    if eng.active:
        if n not in eng.active.src:
            eng.active.src.add(n)
            n.obs.add(eng.active)
            eng.active.depth = max(eng.active.depth, n.depth + 1)

def batch_start():
    eng.batch += 1

def batch_end():
    eng.batch -= 1
    if eng.batch == 0:
        propagate()