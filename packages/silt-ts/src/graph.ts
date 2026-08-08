let sid = 0;

export class CyclicDependencyError extends Error {
    constructor(id: number) {
        super(`Cyclic dependency detected at node ${id}`);
        this.name = "CyclicDependencyError";
    }
}

class BucketQueue {
    private buckets = new Map<number, Node[]>();
    private pending = new Set<Node>();
    private min = Infinity;
    private max = -1;

    push(n: Node) {
        if (this.pending.has(n)) return;
        this.pending.add(n);
        const d = n.d;
        if (!this.buckets.has(d)) this.buckets.set(d, []);
        this.buckets.get(d)!.push(n);
        if (d < this.min) this.min = d;
        if (d > this.max) this.max = d;
    }

    pop(): Node | null {
        while (this.min <= this.max) {
            const b = this.buckets.get(this.min);
            if (b && b.length > 0) {
                const n = b.shift()!;
                this.pending.delete(n);
                if (b.length === 0) {
                    this.buckets.delete(this.min);
                    this.min++;
                }
                return n;
            }
            this.min++;
        }
        this.min = Infinity;
        this.max = -1;
        return null;
    }
}

export const ctx = {
    act: null as Node | null,
    bq: new BucketQueue(),
    b: 0
};

export class Node {
    id = sid++;
    obs = new Set<Node>();
    src = new Set<Node>();
    d = 0;
    st = 0;
    evaluating = false;
    
    update() {}
}

export function link(n: Node) {
    if (ctx.act && !ctx.act.src.has(n)) {
        ctx.act.src.add(n);
        n.obs.add(ctx.act);
        ctx.act.d = Math.max(ctx.act.d, n.d + 1);
    }
}

export function prop() {
    if (ctx.b > 0) return;
    let n: Node | null;
    while ((n = ctx.bq.pop()) !== null) {
        if (n.st === 2) n.update();
        n.st = 0;
    }
}

export class Sig<T> extends Node {
    constructor(public v: T) { super(); }

    get(): T {
        link(this);
        return this.v;
    }

    set(v: T) {
        if (this.v !== v) {
            this.v = v;
            for (const o of this.obs) {
                o.st = 2;
                ctx.bq.push(o);
            }
            prop();
        }
    }
}

export class Computed<T> extends Node {
    private fn: () => T;
    private v!: T;
    err: Error | null = null;
    dirty = true;

    constructor(fn: () => T) {
        super();
        this.fn = fn;
    }

    update() {
        if (this.evaluating) throw new CyclicDependencyError(this.id);
        
        this.st = 0;
        const old = this.v;

        for (const s of this.src) s.obs.delete(this);
        this.src.clear();

        const prev = ctx.act;
        ctx.act = this;
        this.evaluating = true;

        try {
            this.v = this.fn();
            this.err = null;
        } catch (e: any) {
            this.err = e;
        } finally {
            this.evaluating = false;
            ctx.act = prev;
        }

        if (old !== this.v || this.dirty) {
            this.dirty = false;
            for (const o of this.obs) {
                o.st = 2;
                ctx.bq.push(o);
            }
        }
    }

    get(): T {
        if (this.dirty || this.st > 0) this.update();
        if (this.err) throw this.err;
        link(this);
        return this.v;
    }
}