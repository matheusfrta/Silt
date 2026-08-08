import { Sig, Effect } from 'silt';

export function createPersistent<T>(key: string, init: T): Sig<T> {
    const stored = localStorage.getItem(key);
    let val = init;
    
    if (stored !== null) {
        try {
            val = JSON.parse(stored);
        } catch {}
    }
    
    const sig = new Sig<T>(val);
    
    new Effect(() => {
        localStorage.setItem(key, JSON.stringify(sig.get()));
    });
    
    return sig;
}