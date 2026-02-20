<script lang="ts">
    import { auth } from "$lib/stores/authStore";
    import { goto } from "$app/navigation";
    import { login } from "$lib/api/auth";

    let username = "";
    let password = "";
    let error = "";

    async function handleLogin() {
        error = "";
        try {
            const resp = await login(username, password);
            if (resp.ok) {
                const data = await resp.json();
                auth.set(data);
                goto("/");
            } else {
                error = "Invalid username or password";
            }
        } catch (e) {
            error = "Failed to connect to server";
        }
    }
</script>

<div class="auth-page">
    <div class="auth-card">
        <h1>Login</h1>
        {#if error}
            <div class="error">{error}</div>
        {/if}
        <form
            onsubmit={(e) => {
                e.preventDefault();
                handleLogin();
            }}
        >
            <div class="field">
                <label for="username">Username</label>
                <input
                    type="text"
                    id="username"
                    bind:value={username}
                    required
                />
            </div>
            <div class="field">
                <label for="password">Password</label>
                <input
                    type="password"
                    id="password"
                    bind:value={password}
                    required
                />
            </div>
            <button type="button" class="auth-btn" onclick={handleLogin}
                >Sign In</button
            >
        </form>
        <p class="switch">
            Don't have an account? <a href="/signup">Sign Up</a>
        </p>
    </div>
</div>

<style>
    .auth-page {
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: calc(100vh - 56px);
        background: #121212;
    }
    .auth-card {
        background: #1e1e1e;
        padding: 2.5rem;
        border-radius: 12px;
        width: 100%;
        max-width: 400px;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
    }
    h1 {
        margin: 0 0 1.5rem;
        text-align: center;
        color: #fff;
    }
    .field {
        margin-bottom: 1.25rem;
    }
    label {
        display: block;
        margin-bottom: 0.5rem;
        color: #aaa;
        font-size: 0.9rem;
    }
    input {
        width: 100%;
        padding: 0.75rem;
        background: #2a2a2a;
        border: 1px solid #333;
        border-radius: 6px;
        color: #fff;
    }
    .auth-btn {
        width: 100%;
        padding: 0.8rem;
        background: #4caf50;
        color: white;
        border: none;
        border-radius: 6px;
        font-weight: 600;
        cursor: pointer;
        margin-top: 1rem;
    }
    .error {
        color: #f44336;
        margin-bottom: 1rem;
        text-align: center;
        font-size: 0.9rem;
    }
    .switch {
        text-align: center;
        margin-top: 1.5rem;
        color: #888;
        font-size: 0.9rem;
    }
    .switch a {
        color: #4caf50;
        text-decoration: none;
    }
</style>
