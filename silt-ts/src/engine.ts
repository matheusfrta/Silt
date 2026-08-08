export class Node {
    static _id = 0;
    id: number;
    obs: Set<Node> = new Set();
    src: Set<Node> = new Set();
    depth = 0;
    state = 0;

    constructor() {
        this.id = Node._id++;
    }
    update() {}
}

export const eng = {
    active: null as Node | null,
    batch: 0,
    q: [] as Node[]
};

export function propagate() {
    if (eng.batch > 0) return;
    eng.q.sort((a, b) => a.depth - b.depth);
    
    while (eng.q.length > 0) {
        const n = eng.q.shift()!;
        if (n.state === 2) {
            n.update();
        } else if (n.state === 1) {
            for (const s of n.src) {
                if (s.state > 0) s.update();
            }
            if (n.state === 2) n.update();
        }
        n.state = 0;
    }
}

export function link(n: Node) {
    if (eng.active) {
        if (!eng.active.src.has(n)) {
            eng.active.src.add(n);
            n.obs.add(eng.active);
            eng.active.depth = Math.max(eng.active.depth, n.depth + 1);
        }
    }
}

export function batchStart() {
    eng.batch++;
}

export function batchEnd() {
    eng.batch--;
    if (eng.batch === 0) propagate();
}