<script lang="ts">
    import { createNewGame } from "$lib/core/ws";
    import { goto } from "$app/navigation";
    import { auth } from "$lib/stores/authStore";

    async function handleCreateGame(side: "white" | "black" | "random") {
        try {
            const id = await createNewGame(side);
            localStorage.setItem(`game_is_creator_${id}`, "true");
            localStorage.setItem(`game_chosen_side_${id}`, side);
            goto(`/game/${id}`);
        } catch (e) {
            console.error("Failed to create game:", e);
        }
    }
</script>

<svelte:head>
    <title>LajuChess — Welcome</title>
</svelte:head>

<div class="lobby">
    <div class="hero">
        <h1>♟ LajuChess</h1>
        <p>Premium, lightweight, high-performance chess.</p>

        {#if $auth}
            <div class="match-section">
                <h2>Competitive Play</h2>
                <button class="match-btn" on:click={() => goto("/matchmaking")}>
                    <span class="icon">🏆</span>
                    Find a Match
                </button>
            </div>
        {/if}

        <div class="create-section">
            <h2>Start a New Game</h2>
            <div class="side-picker">
                <button
                    class="side-btn white"
                    on:click={() => handleCreateGame("white")}
                    title="Play as White"
                >
                    <svg viewBox="0 0 45 45"
                        ><path
                            d="M22.5 9c-2.21 0-4 1.79-4 4 0 .89.29 1.71.78 2.38C17.33 16.5 16 18.59 16 21c0 2.03.94 3.84 2.41 5.03C15.41 27.09 11 31.58 11 39.5h23c0-7.92-4.41-12.41-7.41-13.47 1.47-1.19 2.41-3 2.41-5.03 0-2.41-1.33-4.5-3.28-5.62.49-.67.78-1.49.78-2.38 0-2.21-1.79-4-4-4z"
                            fill="#fff"
                            stroke="#000"
                            stroke-width="1.5"
                        /></svg
                    >
                </button>
                <button
                    class="side-btn random"
                    on:click={() => handleCreateGame("random")}
                    title="Random Side"
                >
                    <div class="split-icon"></div>
                </button>
                <button
                    class="side-btn black"
                    on:click={() => handleCreateGame("black")}
                    title="Play as Black"
                >
                    <svg viewBox="0 0 45 45"
                        ><path
                            d="M22.5 9c-2.21 0-4 1.79-4 4 0 .89.29 1.71.78 2.38C17.33 16.5 16 18.59 16 21c0 2.03.94 3.84 2.41 5.03C15.41 27.09 11 31.58 11 39.5h23c0-7.92-4.41-12.41-7.41-13.47 1.47-1.19 2.41-3 2.41-5.03 0-2.41-1.33-4.5-3.28-5.62.49-.67.78-1.49.78-2.38 0-2.21-1.79-4-4-4z"
                            fill="#000"
                            stroke="#000"
                            stroke-width="1.5"
                        /></svg
                    >
                </button>
            </div>
        </div>

        <div class="features">
            <div class="feature">
                <span>⚡</span>
                <h3>Fast</h3>
                <p>Rust backend, no bloat.</p>
            </div>
            <div class="feature">
                <span>🤝</span>
                <h3>Shared</h3>
                <p>Play with anyone via link.</p>
            </div>
            <div class="feature">
                <span>🪶</span>
                <h3>Light</h3>
                <p>Low memory, high FPS.</p>
            </div>
        </div>
    </div>
</div>

<style>
    .lobby {
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: calc(100vh - 56px);
        background: radial-gradient(circle at center, #1e1e1e 0%, #121212 100%);
        color: #eee;
        padding: 2rem;
    }
    .hero {
        text-align: center;
        max-width: 800px;
    }
    h1 {
        font-size: 4rem;
        margin: 0 0 1rem;
        background: linear-gradient(to right, #fff, #888);
        -webkit-background-clip: text;
        background-clip: text;
        -webkit-text-fill-color: transparent;
    }
    p {
        font-size: 1.2rem;
        color: #aaa;
        margin-bottom: 3rem;
    }
    .create-section {
        background: rgba(255, 255, 255, 0.05);
        padding: 2.5rem 3rem;
        border-radius: 16px;
        border: 1px solid rgba(255, 255, 255, 0.1);
        margin: 2rem 0;
    }
    .create-section h2 {
        font-size: 1.1rem;
        color: #888;
        margin: 0 0 1.5rem;
        text-transform: uppercase;
        letter-spacing: 2px;
    }
    .side-picker {
        display: flex;
        gap: 1.5rem;
        justify-content: center;
    }
    .side-btn {
        width: 100px;
        height: 100px;
        border: none;
        border-radius: 12px;
        cursor: pointer;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
        display: flex;
        align-items: center;
        justify-content: center;
        background: #2a2a2a;
        padding: 0;
        box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
    }
    .side-btn:hover {
        transform: translateY(-5px) scale(1.05);
        background: #333;
        box-shadow: 0 8px 20px rgba(0, 0, 0, 0.4);
    }
    .side-btn svg {
        width: 65%;
        height: 65%;
    }
    .side-btn.random {
        position: relative;
        overflow: hidden;
    }
    .split-icon {
        width: 100%;
        height: 100%;
        background: linear-gradient(135deg, #dee3e6 50%, #1e1e1e 50%);
    }
    .match-section {
        background: linear-gradient(
            135deg,
            rgba(76, 175, 80, 0.1) 0%,
            rgba(255, 255, 255, 0.05) 100%
        );
        padding: 2.5rem 3rem;
        border-radius: 16px;
        border: 1px solid rgba(76, 175, 80, 0.2);
        margin: 2rem 0;
        text-align: center;
    }
    .match-section h2 {
        font-size: 1.1rem;
        color: #4caf50;
        margin: 0 0 1.5rem;
        text-transform: uppercase;
        letter-spacing: 2px;
    }
    .match-btn {
        background: #4caf50;
        color: white;
        border: none;
        padding: 1rem 2.5rem;
        font-size: 1.25rem;
        font-weight: 700;
        border-radius: 12px;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 0.75rem;
        margin: 0 auto;
        box-shadow: 0 4px 15px rgba(76, 175, 80, 0.3);
        transition: all 0.2s;
    }
    .match-btn:hover {
        background: #45a049;
        transform: translateY(-2px);
        box-shadow: 0 8px 25px rgba(76, 175, 80, 0.4);
    }
    .features {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 2rem;
        margin-top: 5rem;
    }
    .feature {
        background: rgba(255, 255, 255, 0.03);
        padding: 1.5rem;
        border-radius: 12px;
        border: 1px solid rgba(255, 255, 255, 0.05);
    }
    .feature span {
        font-size: 2rem;
        display: block;
        margin-bottom: 1rem;
    }
    .feature h3 {
        margin: 0 0 0.5rem;
        font-size: 1.1rem;
    }
    .feature p {
        font-size: 0.9rem;
        margin: 0;
        color: #888;
    }

    @media (max-width: 600px) {
        h1 {
            font-size: 2.5rem;
        }
        .features {
            grid-template-columns: 1fr;
        }
    }
</style>
