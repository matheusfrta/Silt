let sid = 0;
export const ctx = { act: null as Node | null, q: [] as Node[], b: 0 };

export class Node {
    id = sid++;
    obs = new Set<Node>();
    src = new Set<Node>();
    d = 0;
    st = 0;
    
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
    ctx.q.sort((a, b) => a.d - b.d);
    while (ctx.q.length) {
        const n = ctx.q.shift()!;
        if (n.st === 2) n.update();
        n.st = 0;
    }
}

export class Sig<T> extends Node {
    constructor(public v: T) { super(); }
    get() { link(this); return this.v; }
    set(v: T) {
        if (this.v !== v) {
            this.v = v;
            for (const o of this.obs) {
                o.st = 2;
                if (!ctx.q.includes(o)) ctx.q.push(o);
            }
            prop();
        }
    }
}