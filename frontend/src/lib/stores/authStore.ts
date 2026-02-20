import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export interface User {
    username: string;
    token: string;
}

const initialUser = browser ? JSON.parse(localStorage.getItem('auth_user') || 'null') : null;

export const auth = writable<User | null>(initialUser);

if (browser) {
    auth.subscribe((u) => {
        if (u) {
            localStorage.setItem('auth_user', JSON.stringify(u));
        } else {
            localStorage.removeItem('auth_user');
        }
    });
}

export function logout() {
    auth.set(null);
}
