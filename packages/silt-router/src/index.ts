import { Sig, Computed } from 'silt';
import { track } from 'silt-view';

export const path = new Sig(window.location.pathname);

window.addEventListener('popstate', () => {
    path.set(window.location.pathname);
});

export function navigate(url: string) {
    window.history.pushState(null, '', url);
    path.set(url);
}

export function Route(props: { path: string, component: any }) {
    const match = new Computed(() => path.get() === props.path);
    
    // dom renderer will handle the reactive function
    return () => {
        if (match.get()) {
            return track(() => props.component({}));
        }
        return null;
    };
}