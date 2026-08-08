import { Node, eng, link, propagate } from './engine';

export class Signal<T> extends Node {
    val: T;

    constructor(val: T) {
        super();
        this.val = val;
    }

    get(): T {
        link(this);
        return this.val;
    }

    set(val: T) {
        if (this.val !== val) {
            this.val = val;
            for (const o of this.obs) {
                o.state = 2;
                if (!eng.q.includes(o)) eng.q.push(o);
            }
            propagate();
        }
    }
}

export class Computed<T> extends Node {
    fn: () => T;
    val!: T;
    dirty = true;

    constructor(fn: () => T) {
        super();
        this.fn = fn;
    }

    update() {
        this.state = 0;
        const old = this.val;
        
        for (const s of this.src) s.obs.delete(this);
        this.src.clear();

        const prev = eng.active;
        eng.active = this;
        try {
            this.val = this.fn();
        } finally {
            eng.active = prev;
        }

        if (old !== this.val || this.dirty) {
            this.dirty = false;
            for (const o of this.obs) {
                o.state = 2;
                if (!eng.q.includes(o)) eng.q.push(o);
            }
        }
    }

    get(): T {
        if (this.dirty || this.state > 0) this.update();
        link(this);
        return this.val;
    }
}

export class Effect extends Node {
    fn: () => void;

    constructor(fn: () => void) {
        super();
        this.fn = fn;
        this.update();
    }

    update() {
        this.state = 0;
        for (const s of this.src) s.obs.delete(this);
        this.src.clear();

        const prev = eng.active;
        eng.active = this;
        try {
            this.fn();
        } finally {
            eng.active = prev;
        }
    }
}