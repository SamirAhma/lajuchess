export async function joinMatchmaking(token: string) {
    return fetch("/api/match/join", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ token }),
    });
}

export async function checkMatchStatus(token: string) {
    return fetch("/api/match/status", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ token }),
    });
}
