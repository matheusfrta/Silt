import { Sig, Computed, Effect } from 'silt';
import { track } from 'silt-view';

export function jsx(tag: any, props: any) {
    if (typeof tag === 'function') {
        return track(() => tag(props));
    }

    const el = document.createElement(tag);
    
    for (const k in props) {
        if (k === 'children') {
            insert(el, props[k]);
        } else if (k.startsWith('on')) {
            el.addEventListener(k.slice(2).toLowerCase(), props[k]);
        } else {
            const v = props[k];
            if (typeof v === 'function' || v instanceof Sig || v instanceof Computed) {
                new Effect(() => el.setAttribute(k, typeof v === 'function' ? v() : v.get()));
            } else {
                el.setAttribute(k, v);
            }
        }
    }
    return el;
}

export const jsxs = jsx;

function insert(parent: HTMLElement, child: any) {
    if (Array.isArray(child)) {
        for (const c of child) insert(parent, c);
    } else if (typeof child === 'function' || child instanceof Sig || child instanceof Computed) {
        const marker = document.createTextNode('');
        parent.appendChild(marker);
        let last: Node[] = [];
        
        new Effect(() => {
            last.forEach(n => n.parentNode?.removeChild(n));
            const v = typeof child === 'function' ? child() : child.get();
            
            if (v instanceof HTMLElement) {
                parent.insertBefore(v, marker);
                last = [v];
            } else {
                const n = document.createTextNode(String(v));
                parent.insertBefore(n, marker);
                last = [n];
            }
        });
    } else if (child instanceof HTMLElement || child instanceof Text) {
        parent.appendChild(child);
    } else {
        parent.appendChild(document.createTextNode(String(child)));
    }
}