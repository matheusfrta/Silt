import { useState, useEffect } from 'react';
import { Sig, Node, ctx } from 'silt';

class HookNode extends Node {
    constructor(public cb: () => void) { super(); }
    update() { this.cb(); }
}

export function useSilt<T>(sig: Sig<T>): T {
    const [, render] = useState({});
    
    useEffect(() => {
        const n = new HookNode(() => render({}));
        const prev = ctx.act;
        ctx.act = n;
        sig.get();
        ctx.act = prev;
        
        return () => {
            for (const s of n.src) s.obs.delete(n);
        };
    }, [sig]);

    return sig.get();
}