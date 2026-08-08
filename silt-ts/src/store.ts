import { Signal } from './primitives';

export function createStore<T extends object>(init: T): T {
    const sigs: Record<string | symbol, Signal<any>> = {};
    
    for (const key of Object.keys(init)) {
        sigs[key] = new Signal((init as any)[key]);
    }

    return new Proxy(init, {
        get(t, p) {
            if (sigs[p]) return sigs[p].get();
            return Reflect.get(t, p);
        },
        set(t, p, val) {
            if (sigs[p]) {
                sigs[p].set(val);
            } else {
                sigs[p] = new Signal(val);
            }
            return true;
        }
    });
}