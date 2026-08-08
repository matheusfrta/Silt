import asyncio
from .primitives import Signal, Node
from .engine import link, eng, propagate

class Resource(Node):
    def __init__(self, fetcher):
        super().__init__()
        self.fetcher = fetcher
        self.data = Signal(None)
        self.loading = Signal(False)
        self.error = Signal(None)

    def fetch(self, *args):
        self.loading.set(True)
        self.error.set(None)
        
        coro = self.fetcher(*args)
        
        async def wrap():
            try:
                res = await coro
                self.data.set(res)
            except Exception as e:
                self.error.set(e)
            finally:
                self.loading.set(False)
                
        asyncio.create_task(wrap())

    def get(self):
        link(self)
        return self.data.get()