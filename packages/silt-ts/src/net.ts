import { Sig } from './graph';

export class SiltLink {
    ws: WebSocket;
    sigs = new Map<number, Sig<any>>();

    constructor(url: string) {
        this.ws = new WebSocket(url);
        this.ws.onmessage = (e) => {
            const { id, v } = JSON.parse(e.data);
            if (this.sigs.has(id)) this.sigs.get(id)!.set(v);
        };
    }

    sync(id: number, sig: Sig<any>) {
        this.sigs.set(id, sig);
    }
}