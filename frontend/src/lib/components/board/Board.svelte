<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let fen: string;
    export let orientation: "white" | "black" = "white";

    const dispatch = createEventDispatcher<{ move: string }>();

    let selectedSquare: string | null = null;

    const SVG_PIECES: Record<string, string> = {
        K: '<g fill="none" fill-rule="evenodd" stroke="#000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path stroke-linejoin="miter" d="M22.5 11.63V6M20 8h5"/><path fill="#fff" stroke-linecap="butt" stroke-linejoin="miter" d="M22.5 25s4.5-7.5 3-10.5c0 0-1-2.5-3-2.5s-3 2.5-3 2.5c-1.5 3 3 10.5 3 10.5"/><path fill="#fff" d="M12.5 37c5.5 3.5 14.5 3.5 20 0v-7s9-4.5 6-10.5c-4-6.5-13.5-3.5-16 4V27v-3.5c-2.5-7.5-12-10.5-16-4-3 6 6 10.5 6 10.5v7"/><path d="M12.5 30c5.5-3 14.5-3 20 0m-20 3.5c5.5-3 14.5-3 20 0m-20 3.5c5.5-3 14.5-3 20 0"/></g>',
        Q: '<g fill="#fff" stroke="#000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M9 26c8.5-1.5 21-1.5 27 0l2.5-12.5L31 25l-.3-14.1-5.2 13.6-3-14.5-3 14.5-5.2-13.6-.3 14.1L6.5 13.5 9 26z" stroke-linecap="butt"/><path d="M9 26c0 2 1.5 2 2.5 4 1 1.5 1 1 .5 3.5-1.5 1-1 2.5-1 2.5-1.5 1.5 0 2.5 0 2.5 6.5 1 16.5 1 23 0 0 0 1.5-1 0-2.5 0 0 .5-1.5-1-2.5-.5-2.5-.5-2 .5-3.5 1-2 2.5-2 2.5-4-8.5-1.5-18.5-1.5-27 0z"/><path d="M11.5 30c3.5-1 18.5-1 22 0M12 33.5c6-1 15-1 21 0" fill="none"/><circle cx="6" cy="12" r="2"/><circle cx="14" cy="9" r="2"/><circle cx="22.5" cy="8" r="2"/><circle cx="31" cy="9" r="2"/><circle cx="39" cy="12" r="2"/></g>',
        R: '<g fill="#fff" fill-rule="evenodd" stroke="#000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M9 39h27v-3H9v3zM12 36v-4h21v4H12zM11 14V9h4v2h5V9h5v2h5V9h4v5" stroke-linecap="butt"/><path d="M34 14l-3 3H14l-3-3M31 17v12.5H14V17" stroke-linecap="butt" stroke-linejoin="miter"/><path d="M31 29.5l1.5 2.5h-20l1.5-2.5M11 14h23" fill="none" stroke-linejoin="miter"/></g>',
        B: '<g fill="none" fill-rule="evenodd" stroke="#000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><g fill="#fff" stroke-linecap="butt"><path d="M9 36c3.39-.97 10.11.43 13.5-2 3.39 2.43 10.11 1.03 13.5 2 0 0 1.65.54 3 2-.68.97-1.65.99-3 .5-3.39-.97-10.11.46-13.5-1-3.39 1.46-10.11.03-13.5 1-1.35.49-2.32.47-3-.5 1.35-1.46 3-2 3-2z"/><path d="M15 32c2.5 2.5 12.5 2.5 15 0 .5-1.5 0-2 0-2 0-2.5-2.5-4-2.5-4 5.5-1.5 6-11.5-5-15.5-11 4-10.5 14-5 15.5 0 0-2.5 1.5-2.5 4 0 0-.5.5 0 2z"/><path d="M25 8a2.5 2.5 0 1 1-5 0 2.5 2.5 0 1 1 5 0z"/></g><path d="M17.5 26h10M15 30h15m-7.5-14.5v5m-2.5-2.5h5" stroke-linejoin="miter"/></g>',
        N: '<g fill="none" fill-rule="evenodd" stroke="#000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 10c10.5 1 16.5 8 16 29H15c0-9 10-6.5 8-21" fill="#fff"/><path d="M24 18c.38 2.91-5.55 7.37-8 9-3 2-2.82 4.34-5 4-1.04-.94 1.41-3.04 0-3-1 0 .19 1.23-1 2-1 0-4 1-4-4 0-2 6-12 6-12s1.89-1.9 2-3.5c-.73-.99-.5-2-.5-3 1-1 3 2.5 3 2.5h2s.78-1.99 2.5-3c1 0 1 3 1 3" fill="#fff"/><path d="M9.5 25.5a.5.5 0 1 1-1 0 .5.5 0 1 1 1 0z" fill="#000"/><path d="M15 15.5a.5 1.5 0 1 1-1 0 .5 1.5 0 1 1 1 0z" transform="matrix(.866 .5 -.5 .866 9.693 -5.173)" fill="#000"/></g>',
        P: '<path d="M22.5 9c-2.21 0-4 1.79-4 4 0 .89.29 1.71.78 2.38C17.33 16.5 16 18.59 16 21c0 2.03.94 3.84 2.41 5.03C15.41 27.09 11 31.58 11 39.5h23c0-7.92-4.41-12.41-7.41-13.47 1.47-1.19 2.41-3 2.41-5.03 0-2.41-1.33-4.5-3.28-5.62.49-.67.78-1.49.78-2.38 0-2.21-1.79-4-4-4z" fill="#fff" stroke="#000" stroke-width="1.5" stroke-linecap="round"/>',
        k: '<g fill="none" stroke="#000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22.5 11.63V6M20 8h5" stroke-linejoin="miter"/><path d="M22.5 25s4.5-7.5 3-10.5c0 0-1-2.5-3-2.5s-3 2.5-3 2.5C18 17.5 22.5 25 22.5 25" fill="#000" stroke-linecap="butt" stroke-linejoin="miter"/><path d="M12.5 37c5.5 3.5 14.5 3.5 20 0v-7s9-4.5 6-10.5c-4-6.5-13.5-3.5-16 4V27v-3.5C20 16 10.5 13 6.5 19.5 3.5 25.5 12.5 30 12.5 30l0 7" fill="#000"/><path d="M32 29.5s8.5-4 6.03-9.65C34.15 14 25 18 22.5 24.5v2.1v-2.1c-2.5-6.5-11.65-10.5-15.53-4.65C4.5 25.5 13 29.5 13 29.5" stroke="#fff"/><path d="M12.5 30c5.5-3 14.5-3 20 0m-20 3.5c5.5-3 14.5-3 20 0m-20 3.5c5.5-3 14.5-3 20 0" stroke="#fff"/></g>',
        q: '<g fill="#000" stroke="#000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M9 26c8.5-1.5 21-1.5 27 0l2.5-12.5L31 25l-.3-14.1-5.2 13.6-3-14.5-3 14.5-5.2-13.6-.3 14.1L6.5 13.5 9 26z" stroke-linecap="butt"/><path d="M9 26c0 2 1.5 2 2.5 4 1 1.5 1 1 .5 3.5-1.5 1-1 2.5-1 2.5-1.5 1.5 0 2.5 0 2.5 6.5 1 16.5 1 23 0 0 0 1.5-1 0-2.5 0 0 .5-1.5-1-2.5-.5-2.5-.5-2 .5-3.5 1-2 2.5-2 2.5-4-8.5-1.5-18.5-1.5-27 0z"/><path d="M11.5 30c3.5-1 18.5-1 22 0M12 33.5c6-1 15-1 21 0M11 38.5a35 35 1 0 0 23 0" fill="none" stroke-linecap="butt"/><circle cx="6" cy="12" r="2"/><circle cx="14" cy="9" r="2"/><circle cx="22.5" cy="8" r="2"/><circle cx="31" cy="9" r="2"/><circle cx="39" cy="12" r="2"/><g fill="none" stroke="#fff"><path d="M11 29a35 35 1 0 1 23 0M12.5 31.5h20m-21 3a35 35 1 0 0 22 0m-23 3a35 35 1 0 0 24 0"/></g></g>',
        r: '<g fill="#000" fill-rule="evenodd" stroke="#000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M9 39h27v-3H9v3zM12.5 32l1.5-2.5h17l1.5 2.5h-20zM12 36v-4h21v4H12zM14 29.5v-13h17v13h-17z" stroke-linecap="butt"/><path d="M14 16.5L11 14h23l-3 2.5H14zM11 14V9h4v2h5V9h5v2h5V9h4v5H11z" stroke-linecap="butt"/><path d="M12 35.5h21M13 31.5h19M14 29.5h17M14 16.5h17M11 14h23" fill="none" stroke="#fff" stroke-width="1" stroke-linejoin="miter"/></g>',
        b: '<g fill="none" fill-rule="evenodd" stroke="#000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><g fill="#000" stroke-linecap="butt"><path d="M9 36c3.39-.97 10.11.43 13.5-2 3.39 2.43 10.11 1.03 13.5 2 0 0 1.65.54 3 2-.68.97-1.65.99-3 .5-3.39-.97-10.11.46-13.5-1-3.39 1.46-10.11.03-13.5 1-1.35.49-2.32.47-3-.5 1.35-1.46 3-2 3-2z"/><path d="M15 32c2.5 2.5 12.5 2.5 15 0 .5-1.5 0-2 0-2 0-2.5-2.5-4-2.5-4 5.5-1.5 6-11.5-5-15.5-11 4-10.5 14-5 15.5 0 0-2.5 1.5-2.5 4 0 0-.5.5 0 2z"/><path d="M25 8a2.5 2.5 0 1 1-5 0 2.5 2.5 0 1 1 5 0z"/></g><path d="M17.5 26h10M15 30h15m-7.5-14.5v5m-2.5-2.5h5" stroke="#fff" stroke-linejoin="miter"/></g>',
        n: '<g fill="none" fill-rule="evenodd" stroke="#000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 10c10.5 1 16.5 8 16 29H15c0-9 10-6.5 8-21" fill="#000"/><path d="M24 18c.38 2.91-5.55 7.37-8 9-3 2-2.82 4.34-5 4-1.04-.94 1.41-3.04 0-3-1 0 .19 1.23-1 2-1 0-4 1-4-4 0-2 6-12 6-12s1.89-1.9 2-3.5c-.73-.99-.5-2-.5-3 1-1 3 2.5 3 2.5h2s.78-1.99 2.5-3c1 0 1 3 1 3" fill="#000"/><path d="M9.5 25.5a.5.5 0 1 1-1 0 .5.5 0 1 1 1 0z" fill="#fff" stroke="#fff"/><path d="M15 15.5a.5 1.5 0 1 1-1 0 .5 1.5 0 1 1 1 0z" transform="matrix(.866 .5 -.5 .866 9.693 -5.173)" fill="#fff" stroke="#fff"/><path d="M24.55 10.4l-.45 1.45.5.15c3.15 1 5.65 2.49 7.9 6.75 2.25 4.26 3.25 10.31 2.75 20.25l-.05.5h2.25l.05-.5c.5-10.06-.88-16.85-3.25-21.34-2.37-4.49-5.79-6.64-9.19-7.16l-.51-.1z" fill="#fff" stroke="none"/></g>',
        p: '<path d="M22.5 9c-2.21 0-4 1.79-4 4 0 .89.29 1.71.78 2.38C17.33 16.5 16 18.59 16 21c0 2.03.94 3.84 2.41 5.03C15.41 27.09 11 31.58 11 39.5h23c0-7.92-4.41-12.41-7.41-13.47 1.47-1.19 2.41-3 2.41-5.03 0-2.41-1.33-4.5-3.28-5.62.49-.67.78-1.49.78-2.38 0-2.21-1.79-4-4-4z" fill="#000" stroke="#000" stroke-width="1.5" stroke-linecap="round"/>',
    };

    interface Square {
        name: string;
        piece: string | null;
        isLight: boolean;
        row: number;
        col: number;
    }

    function parseFen(fen: string): Square[] {
        const squares: Square[] = [];
        const rows = fen.split(" ")[0].split("/");
        rows.forEach((row, rowIdx) => {
            let colIdx = 0;
            for (const char of row) {
                if (isNaN(Number(char))) {
                    const name =
                        String.fromCharCode(97 + colIdx) + (8 - rowIdx);
                    squares.push({
                        name,
                        piece: char,
                        isLight: (rowIdx + colIdx) % 2 === 0,
                        row: rowIdx,
                        col: colIdx,
                    });
                    colIdx++;
                } else {
                    for (let i = 0; i < Number(char); i++) {
                        const name =
                            String.fromCharCode(97 + colIdx) + (8 - rowIdx);
                        squares.push({
                            name,
                            piece: null,
                            isLight: (rowIdx + colIdx) % 2 === 0,
                            row: rowIdx,
                            col: colIdx,
                        });
                        colIdx++;
                    }
                }
            }
        });
        return squares;
    }

    $: squares = parseFen(fen);
    $: displayedSquares =
        orientation === "white" ? squares : [...squares].reverse();

    function handleClick(sq: Square) {
        if (selectedSquare === null) {
            if (sq.piece) selectedSquare = sq.name;
        } else {
            if (selectedSquare !== sq.name) {
                dispatch("move", selectedSquare + sq.name);
            }
            selectedSquare = null;
        }
    }

    function handleDragStart(e: DragEvent, sq: Square) {
        if (!sq.piece) return;
        e.dataTransfer?.setData("text/plain", sq.name);
        e.dataTransfer!.effectAllowed = "move";
    }

    function handleDrop(e: DragEvent, sq: Square) {
        e.preventDefault();
        const from = e.dataTransfer?.getData("text/plain");
        if (from && from !== sq.name) dispatch("move", from + sq.name);
    }

    function isWhitePiece(piece: string) {
        return piece === piece.toUpperCase();
    }
</script>

<div class="board-container">
    <div class="board">
        {#each displayedSquares as sq}
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div
                class="square"
                class:light={sq.isLight}
                class:dark={!sq.isLight}
                class:selected={selectedSquare === sq.name}
                draggable={sq.piece !== null}
                on:click={() => handleClick(sq)}
                on:dragstart={(e) => handleDragStart(e, sq)}
                on:dragover|preventDefault
                on:drop={(e) => handleDrop(e, sq)}
            >
                {#if sq.piece}
                    <svg
                        viewBox="0 0 45 45"
                        class="piece"
                        class:white={isWhitePiece(sq.piece)}
                        class:black={!isWhitePiece(sq.piece)}
                    >
                        {@html SVG_PIECES[sq.piece]}
                    </svg>
                {/if}

                <!-- Coordinates -->
                {#if (orientation === "white" && sq.col === 0) || (orientation === "black" && sq.col === 7)}
                    <span
                        class="coord rank"
                        class:light-text={!sq.isLight}
                        class:dark-text={sq.isLight}
                    >
                        {8 - sq.row}
                    </span>
                {/if}
                {#if (orientation === "white" && sq.row === 7) || (orientation === "black" && sq.row === 0)}
                    <span
                        class="coord file"
                        class:light-text={!sq.isLight}
                        class:dark-text={sq.isLight}
                    >
                        {String.fromCharCode(97 + sq.col)}
                    </span>
                {/if}
            </div>
        {/each}
    </div>
</div>

<style>
    .board-container {
        width: min(90vw, 600px);
        margin: 0 auto;
        user-select: none;
    }
    .board {
        display: grid;
        grid-template-columns: repeat(8, 1fr);
        grid-template-rows: repeat(8, 1fr);
        aspect-ratio: 1 / 1;
        border: 4px solid #312e2b;
        border-radius: 4px;
        box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
    }
    .square {
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
    }
    .light {
        background: #dee3e6;
    }
    .dark {
        background: #8ca2ad;
    }

    .square:hover::after {
        content: "";
        position: absolute;
        inset: 0;
        background: rgba(255, 255, 255, 0.15);
    }

    .selected::after {
        content: "";
        position: absolute;
        inset: 0;
        background: rgba(255, 255, 0, 0.3) !important;
    }

    .piece {
        width: 90%;
        height: 90%;
        z-index: 2;
        filter: drop-shadow(0 3px 3px rgba(0, 0, 0, 0.25));
    }

    .piece.white {
        color: #fff;
    }
    .piece.black {
        color: #000;
    }

    /* Coordinates */
    .coord {
        position: absolute;
        font-size: 0.7rem;
        font-weight: 700;
        z-index: 1;
        opacity: 0.8;
    }
    .rank {
        left: 3px;
        top: 3px;
    }
    .file {
        right: 3px;
        bottom: 3px;
    }
    .light-text {
        color: #8ca2ad;
    }
    .dark-text {
        color: #dee3e6;
    }

    @media (max-width: 600px) {
        .coord {
            font-size: 0.5rem;
        }
    }
</style>
