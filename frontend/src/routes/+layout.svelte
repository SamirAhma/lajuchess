<script lang="ts">
	import "../app.css";
	import PerformanceMonitor from "$lib/components/ui/PerformanceMonitor.svelte";
	import { auth, logout } from "$lib/stores/authStore";
</script>

<header>
	<div class="left">
		<a href="/" class="logo">♟ LajuChess</a>
		<PerformanceMonitor />
	</div>
	<nav>
		<a href="/">Play</a>
		{#if $auth}
			<a href="/matchmaking" class="match-link">Matchmaking</a>
			<div class="user-menu">
				<span class="username">{$auth.username}</span>
				<button class="logout-btn" on:click={logout}>Logout</button>
			</div>
		{:else}
			<a href="/login">Login</a>
			<a href="/signup" class="signup-link">Sign Up</a>
		{/if}
		<a href="/about">About</a>
	</nav>
</header>

<main>
	<slot />
</main>

<style>
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 2rem;
		background: #1a1a1a;
		border-bottom: 1px solid #333;
	}
	.left {
		display: flex;
		align-items: center;
		gap: 1.5rem;
	}
	.logo {
		font-size: 1.4rem;
		font-weight: 700;
		color: #fff;
		text-decoration: none;
		letter-spacing: -0.5px;
	}
	nav {
		display: flex;
		gap: 1.5rem;
		align-items: center;
	}
	nav a {
		color: #aaa;
		text-decoration: none;
		font-size: 0.95rem;
		transition: color 0.2s;
	}
	nav a:hover {
		color: #fff;
	}
	main {
		flex: 1;
	}
	.user-menu {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		padding: 0.25rem 0.5rem 0.25rem 0.75rem;
		border-radius: 20px;
		margin-left: 0.5rem;
	}
	.username {
		font-size: 0.85rem;
		font-weight: 600;
		color: #4caf50;
	}
	.logout-btn {
		background: #333;
		border: none;
		color: #eee;
		padding: 0.25rem 0.75rem;
		border-radius: 15px;
		font-size: 0.75rem;
		cursor: pointer;
		transition: background 0.2s;
	}
	.logout-btn:hover {
		background: #444;
	}
	.signup-link {
		background: #4caf50;
		color: white !important;
		padding: 0.4rem 1rem;
		border-radius: 6px;
		font-weight: 600;
	}
	.match-link {
		color: #4caf50 !important;
		font-weight: 600;
	}
</style>
