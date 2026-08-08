from .primitives import Signal

def create_store(obj):
    if isinstance(obj, dict):
        return ReactiveDict(obj)
    if isinstance(obj, list):
        return ReactiveList(obj)
    return obj

class ReactiveDict:
    def __init__(self, raw):
        self._raw = raw
        self._sigs = {}
        
    def __getitem__(self, key):
        if key not in self._sigs:
            val = create_store(self._raw.get(key))
            self._raw[key] = val
            self._sigs[key] = Signal(val)
        return self._sigs[key].get()
        
    def __setitem__(self, key, val):
        wrapped = create_store(val)
        self._raw[key] = wrapped
        if key in self._sigs:
            self._sigs[key].set(wrapped)
        else:
            self._sigs[key] = Signal(wrapped)

class ReactiveList:
    # minimal implementation wrapper
    def __init__(self, raw):
        self._raw = raw
        self._sig = Signal(raw)
        
    def get(self):
        return self._sig.get()
        
    def append(self, val):
        self._raw.append(create_store(val))
        self._sig.set(self._raw[:])