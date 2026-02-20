import { writable } from 'svelte/store';
import type { GameState, ServerMessage, ClientMessage, CreateGameResponse } from '../types';

function createGameStore() {
    const { subscribe, set } = writable<GameState | null>(null);
    const connected = writable(false);
    let socket: WebSocket | null = null;
    let currentId: string | null = null;

    function connect(gameId: string) {
        if (socket) {
            socket.close();
        }
        currentId = gameId;
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        socket = new WebSocket(`${protocol}//${window.location.host}/ws/${gameId}`);

        socket.onopen = () => connected.set(true);

        socket.onmessage = (event) => {
            const msg: ServerMessage = JSON.parse(event.data);
            if (msg.type === 'State') {
                set(msg.payload);
            } else if (msg.type === 'Error') {
                console.error('Server error:', msg.payload);
            }
        };

        socket.onclose = () => {
            connected.set(false);
            if (currentId === gameId) {
                setTimeout(() => connect(gameId), 2000); // auto-reconnect
            }
        };
    }

    function sendMove(uci: string) {
        if (!socket || socket.readyState !== WebSocket.OPEN) return;
        const msg: ClientMessage = { type: 'Move', payload: { uci } };
        socket.send(JSON.stringify(msg));
    }

    return { subscribe, connected, connect, sendMove };
}

export const game = createGameStore();

export async function createNewGame(side: string): Promise<string> {
    const resp = await fetch('/api/games', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ side })
    });
    const data: CreateGameResponse = await resp.json();
    return data.id;
}
