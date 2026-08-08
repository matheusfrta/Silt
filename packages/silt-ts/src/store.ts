import { Sig } from './graph';

export function makeStore<T extends object>(obj: T): T {
    const s = new Map<string, Sig<any>>();
    return new Proxy(obj, {
        get(t, p: string) {
            if (!s.has(p)) s.set(p, new Sig((t as any)[p]));
            return s.get(p)!.get();
        },
        set(t, p: string, v) {
            (t as any)[p] = v;
            if (!s.has(p)) s.set(p, new Sig(v));
            else s.get(p)!.set(v);
            return true;
        }
    });
}