<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { dev } from "$app/environment";
    import { page } from "$app/stores";

    let memory = "0 MB";
    let domNodes = 0;
    let fps = 0;
    let dataIn = "0 KB";
    let lastTime = performance.now();
    let frames = 0;
    let interval: any;

    // Visibile in dev OR if ?perf=1 is in URL
    $: isVisible = dev || $page.url.searchParams.has("perf");

    function updateStats() {
        // Memory (Chrome only)
        if ((performance as any).memory) {
            const used =
                (performance as any).memory.usedJSHeapSize / 1024 / 1024;
            memory = used.toFixed(1) + " MB";
        }

        // DOM Node count
        domNodes = document.getElementsByTagName("*").length;

        // Network estimation (very rough via performance API)
        const resources = performance.getEntriesByType("resource");
        const totalBytes = resources.reduce(
            (acc, res: any) => acc + (res.transferSize || 0),
            0,
        );
        dataIn = (totalBytes / 1024).toFixed(1) + " KB";

        // FPS calculation
        const now = performance.now();
        frames++;
        if (now > lastTime + 1000) {
            fps = Math.round((frames * 1000) / (now - lastTime));
            lastTime = now;
            frames = 0;
        }

        interval = requestAnimationFrame(updateStats);
    }

    onMount(() => {
        interval = requestAnimationFrame(updateStats);
    });

    onDestroy(() => {
        if (interval) cancelAnimationFrame(interval);
    });
</script>

{#if isVisible}
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div
        class="perf-monitor"
        on:click={() => (isVisible = false)}
        title="Click to hide"
    >
        <span title="Memory Usage">🧠 {memory}</span>
        <span title="DOM Nodes">🌳 {domNodes} nodes</span>
        <span title="Frames Per Second">⚡ {fps} fps</span>
        <span title="Network Data">� {dataIn}</span>
    </div>
{/if}

<style>
    .perf-monitor {
        display: flex;
        gap: 1rem;
        font-size: 0.7rem;
        font-family: monospace;
        color: #4caf50;
        background: rgba(0, 0, 0, 0.4);
        padding: 0.3rem 0.8rem;
        border-radius: 4px;
        border: 1px solid rgba(76, 175, 80, 0.3);
        cursor: pointer;
        backdrop-filter: blur(4px);
    }
    span {
        display: flex;
        align-items: center;
        gap: 0.3rem;
    }
</style>
