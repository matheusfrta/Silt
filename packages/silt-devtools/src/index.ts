import { ctx } from 'silt';

export function mountDevTools() {
    if (typeof window === 'undefined') return;

    const bqPush = ctx.bq.push.bind(ctx.bq);
    
    // intercept bucket queue pushes to trace execution
    ctx.bq.push = (n: any) => {
        window.postMessage({ 
            source: 'silt-devtools', 
            type: 'QUEUE_PUSH', 
            id: n.id, 
            depth: n.d,
            state: n.st
        }, '*');
        bqPush(n);
    };

    (window as any).__SILT_ROOT = ctx;
    console.log('[Silt] DevTools injected into bucket queue.');
}