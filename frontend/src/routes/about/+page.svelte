<svelte:head>
	<title>About — LajuChess</title>
</svelte:head>

<div class="page">
	<h1>About LajuChess</h1>
	<p>
		A high-performance chess platform built with <strong>Rust</strong> +
		<strong>SvelteKit</strong>.
	</p>
	<ul>
		<li>⚡ Rust backend — low latency, minimal RAM</li>
		<li>🎯 SvelteKit frontend — fast, reactive, code-split</li>
		<li>🔗 WebSockets for real-time moves</li>
		<li>🔒 Type-safe end-to-end via <code>typeshare</code></li>
	</ul>
</div>

<style>
	.page {
		padding: 3rem 2rem;
		max-width: 640px;
		margin: 0 auto;
		color: #eee;
		background: #121212;
		min-height: calc(100vh - 56px);
	}
	h1 {
		font-size: 2rem;
		margin-bottom: 1rem;
	}
	p,
	li {
		color: #bbb;
		line-height: 1.7;
	}
	ul {
		padding-left: 1.25rem;
	}
	code {
		background: #2a2a2a;
		padding: 0.1em 0.4em;
		border-radius: 3px;
		font-size: 0.9em;
	}
</style>
