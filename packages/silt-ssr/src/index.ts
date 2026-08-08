import { Sig, Computed } from 'silt';

export function renderToString(fn: () => any): string {
    const oldJsx = (globalThis as any).__SILT_JSX;
    
    // mock jsx for fast string concat
    (globalThis as any).__SILT_JSX = (tag: any, props: any) => {
        if (typeof tag === 'function') return tag(props);
        
        let s = `<${tag}`;
        let childStr = '';
        
        for (const k in props) {
            if (k === 'children') {
                const c = props[k];
                childStr = parseChild(c);
            } else if (!k.startsWith('on')) {
                const v = typeof props[k] === 'function' ? props[k]() 
                        : props[k] instanceof Sig || props[k] instanceof Computed ? props[k].get() 
                        : props[k];
                s += ` ${k}="${v}"`;
            }
        }
        s += `>${childStr}</${tag}>`;
        return s;
    };
    
    try {
        return fn();
    } finally {
        (globalThis as any).__SILT_JSX = oldJsx;
    }
}

function parseChild(c: any): string {
    if (Array.isArray(c)) return c.map(parseChild).join('');
    if (typeof c === 'function') return parseChild(c());
    if (c instanceof Sig || c instanceof Computed) return parseChild(c.get());
    return String(c);
}