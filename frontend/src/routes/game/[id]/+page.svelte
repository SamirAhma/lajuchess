<script lang="ts">
    import { onMount } from "svelte";
    import { page } from "$app/stores";
    import { game } from "$lib/core/ws";
    import Board from "$lib/components/board/Board.svelte";

    const id = $page.params.id as string;
    const connected = game.connected;

    let isCreator = false;
    $: orientation = $game
        ? isCreator
            ? ($game.creator_color as "white" | "black")
            : $game.creator_color === "white"
              ? "black"
              : "white"
        : "white";

    $: if ($game) {
        console.log("Game State Sync:", {
            id,
            isCreator,
            creatorColor: $game.creator_color,
            finalOrientation: orientation,
        });
    }

    onMount(() => {
        isCreator = localStorage.getItem(`game_is_creator_${id}`) === "true";
        console.log("Mounting Game:", {
            id,
            isCreator,
        });
        game.connect(id);
    });

    function handleMove(e: CustomEvent<string>) {
        game.sendMove(e.detail);
    }

    function shareLink() {
        navigator.clipboard.writeText(window.location.href);
        alert("Link copied to clipboard!");
    }
</script>

<svelte:head>
    <title>LajuChess — Game {id.slice(0, 8)}</title>
</svelte:head>

<div class="page">
    <div class="board-area">
        {#if $game}
            <Board fen={$game.fen} {orientation} on:move={handleMove} />
        {:else}
            <div class="loading">Connecting to Game…</div>
        {/if}
    </div>

    <aside class="sidebar">
        <div class="status-badge" class:connected={$connected}>
            {$connected ? "● Connected" : "○ Disconnected"}
        </div>

        <section class="game-info">
            <h2>Game ID</h2>
            <div class="id-container">
                <p class="id">{id}</p>
                <div class="actions">
                    <button class="action-btn" on:click={shareLink}
                        >Share Link</button
                    >
                </div>
            </div>
            <p class="side-hint">Playing as <b>{orientation}</b></p>
        </section>

        <section class="game-info">
            <h2>Game State</h2>
            {#if $game}
                <p class="fen"><span>FEN</span> {$game.fen}</p>
            {/if}
        </section>
    </aside>
</div>

<style>
    .page {
        display: flex;
        gap: 2rem;
        padding: 2rem;
        min-height: calc(100vh - 56px);
        background: #121212;
        color: #eee;
        align-items: flex-start;
        justify-content: center;
    }
    .board-area {
        flex-shrink: 0;
    }
    .loading {
        width: min(90vw, 520px);
        aspect-ratio: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        background: #1e1e1e;
        border-radius: 4px;
        color: #666;
        font-size: 1.2rem;
    }
    .sidebar {
        flex: 1;
        max-width: 400px;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        padding-top: 0.5rem;
    }
    .status-badge {
        font-size: 0.85rem;
        font-weight: 600;
        color: #f44336;
        letter-spacing: 0.5px;
    }
    .status-badge.connected {
        color: #4caf50;
    }
    .game-info {
        background: #1e1e1e;
        border-radius: 8px;
        padding: 1.25rem;
    }
    h2 {
        margin: 0 0 0.75rem;
        font-size: 1rem;
        color: #fff;
    }
    .id-container {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }
    .id {
        font-size: 0.8rem;
        color: #888;
        margin: 0;
        font-family: monospace;
    }
    .actions {
        display: flex;
        gap: 0.5rem;
    }
    .action-btn {
        flex: 1;
        background: #333;
        color: #fff;
        border: 1px solid #444;
        padding: 0.6rem;
        border-radius: 4px;
        cursor: pointer;
        font-size: 0.85rem;
        transition: background 0.2s;
    }
    .action-btn:hover {
        background: #444;
    }
    .side-hint {
        margin: 0.75rem 0 0;
        font-size: 0.85rem;
        color: #aaa;
    }
    .side-hint b {
        color: #fff;
        text-transform: capitalize;
    }
    .fen {
        font-family: monospace;
        font-size: 0.8rem;
        color: #aaa;
        word-break: break-all;
        margin: 0;
    }
    .fen span {
        color: #4caf50;
        font-weight: 600;
        margin-right: 0.4rem;
    }

    @media (max-width: 900px) {
        .page {
            flex-direction: column;
            align-items: center;
            padding: 1rem;
        }
        .sidebar {
            width: 100%;
            max-width: 600px;
        }
    }
</style>
