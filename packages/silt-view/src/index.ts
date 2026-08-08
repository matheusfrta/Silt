import { Effect } from 'silt';

class Scope {
    cleanups: (() => void)[] = [];
    effects: Effect[] = [];

    dispose() {
        for (const c of this.cleanups) c();
        for (const e of this.effects) e.stop();
        this.cleanups = [];
        this.effects = [];
    }
}

export const viewCtx = {
    scope: null as Scope | null
};

export function track<T>(fn: () => T): T {
    const parent = viewCtx.scope;
    const s = new Scope();
    viewCtx.scope = s;
    
    try {
        const res = fn();
        if (parent) {
            parent.cleanups.push(() => s.dispose());
        }
        return res;
    } finally {
        viewCtx.scope = parent;
    }
}

export function onCleanup(fn: () => void) {
    if (viewCtx.scope) viewCtx.scope.cleanups.push(fn);
}

export function onMount(fn: () => void) {
    new Effect(() => {
        // queue for microtask to ensure DOM is ready
        Promise.resolve().then(fn);
    });
}