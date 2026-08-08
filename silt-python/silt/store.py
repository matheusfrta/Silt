from .primitives import Signal

class Store:
    def __init__(self, **kwargs):
        self._sigs = {}
        for k, v in kwargs.items():
            self._sigs[k] = Signal(v)

    def __getattr__(self, name):
        if name in self._sigs:
            return self._sigs[name].get()
        return super().__getattribute__(name)

    def __setattr__(self, name, val):
        if name == '_sigs':
            super().__setattr__(name, val)
        elif name in self._sigs:
            self._sigs[name].set(val)
        else:
            self._sigs[name] = Signal(val)