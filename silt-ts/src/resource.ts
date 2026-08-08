import { Signal } from './primitives';

export class Resource<T, Args extends any[] = []> {
    loading = new Signal<boolean>(false);
    error = new Signal<Error | null>(null);
    data: Signal<T | undefined>;
    
    private fetcher: (...args: Args) => Promise<T>;

    constructor(fetcher: (...args: Args) => Promise<T>, init?: T) {
        this.fetcher = fetcher;
        this.data = new Signal<T | undefined>(init);
    }

    async fetch(...args: Args): Promise<T | void> {
        this.loading.set(true);
        this.error.set(null);
        try {
            const res = await this.fetcher(...args);
            this.data.set(res);
            return res;
        } catch (e: any) {
            this.error.set(e);
        } finally {
            this.loading.set(false);
        }
    }
    
    get(): T | undefined {
        return this.data.get();
    }
}