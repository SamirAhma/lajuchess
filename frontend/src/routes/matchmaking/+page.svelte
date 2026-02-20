<script lang="ts">
    import { auth } from "$lib/stores/authStore";
    import { onMount, onDestroy } from "svelte";
    import { goto } from "$app/navigation";
    import { joinMatchmaking, checkMatchStatus } from "$lib/api/match";

    let status = "Initializing...";
    let dotCount = 0;
    let interval: any;

    $: if (!$auth) {
        goto("/login");
    }

    onMount(async () => {
        if ($auth) {
            status = "Joining Matchmaking Queue";
            interval = setInterval(async () => {
                dotCount = (dotCount + 1) % 4;

                // Poll status
                try {
                    const statusResp = await checkMatchStatus(
                        $auth?.token || "",
                    );
                    if (statusResp.ok) {
                        const data = await statusResp.json();
                        if (data.status === "matched") {
                            goto(`/game/${data.game_id}`);
                        }
                    }
                } catch (e) {
                    console.error("Polling failed:", e);
                }
            }, 2000);

            try {
                const resp = await joinMatchmaking($auth?.token || "");
                if (resp.ok) {
                    status = "Waiting for opponent";
                } else {
                    status = "Matchmaking failed";
                }
            } catch (e) {
                status = "Connection error";
            }
        }
    });

    onDestroy(() => {
        if (interval) clearInterval(interval);
    });

    $: displayedStatus = status + ".".repeat(dotCount);
</script>

<div class="matchmaking-page">
    <div class="loader-container">
        <div class="loader"></div>
        <h1>Matchmaking</h1>
        <p class="status">{displayedStatus}</p>
        <p class="hint">We'll notify you as soon as a match is found!</p>
        <button class="cancel-btn" on:click={() => goto("/")}>Cancel</button>
    </div>
</div>

<style>
    .matchmaking-page {
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: calc(100vh - 56px);
        background: radial-gradient(circle at center, #1e1e1e 0%, #121212 100%);
        color: #eee;
    }
    .loader-container {
        text-align: center;
        background: rgba(255, 255, 255, 0.03);
        padding: 4rem;
        border-radius: 20px;
        border: 1px solid rgba(255, 255, 255, 0.05);
        box-shadow: 0 20px 50px rgba(0, 0, 0, 0.5);
    }
    .loader {
        width: 60px;
        height: 60px;
        border: 4px solid #333;
        border-top-color: #4caf50;
        border-radius: 50%;
        animation: spin 1s linear infinite;
        margin: 0 auto 2rem;
    }
    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }
    h1 {
        font-size: 2.5rem;
        margin-bottom: 1rem;
    }
    .status {
        font-size: 1.25rem;
        color: #4caf50;
        font-family: monospace;
        margin: 1rem 0;
    }
    .hint {
        color: #888;
        margin-bottom: 2.5rem;
    }
    .cancel-btn {
        background: transparent;
        border: 1px solid #444;
        color: #888;
        padding: 0.75rem 2rem;
        border-radius: 6px;
        cursor: pointer;
        transition: all 0.2s;
    }
    .cancel-btn:hover {
        background: #333;
        color: #fff;
        border-color: #555;
    }
</style>
