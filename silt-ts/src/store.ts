import { Signal } from './primitives';

export function createStore<T extends object>(init: T): T {
    const raw = Array.isArray(init) ? [] : {};
    const sigs: Map<string | symbol, Signal<any>> = new Map();

    const handler: ProxyHandler<any> = {
        get(t, p) {
            if (!sigs.has(p)) {
                let v = Reflect.get(t, p);
                if (typeof v === 'object' && v !== null) {
                    v = createStore(v);
                    Reflect.set(t, p, v);
                }
                sigs.set(p, new Signal(v));
            }
            return sigs.get(p)!.get();
        },
        set(t, p, val) {
            let v = val;
            if (typeof v === 'object' && v !== null) {
                v = createStore(v);
            }
            Reflect.set(t, p, v);
            
            if (sigs.has(p)) {
                sigs.get(p)!.set(v);
            } else {
                sigs.set(p, new Signal(v));
            }
            return true;
        }
    };

    Object.assign(raw, init);
    return new Proxy(raw, handler);
}