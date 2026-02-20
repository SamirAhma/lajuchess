<script lang="ts">
    import { goto } from "$app/navigation";
    import { signup } from "$lib/api/auth";

    let username = "";
    let password = "";
    let confirmPassword = "";
    let error = "";

    async function handleSignup() {
        if (password !== confirmPassword) {
            error = "Passwords do not match";
            return;
        }
        error = "";
        try {
            const resp = await signup(username, password);
            if (resp.ok) {
                goto("/login");
            } else {
                const msg = await resp.text();
                error = msg || "Signup failed";
            }
        } catch (e) {
            error = "Failed to connect to server";
        }
    }
</script>

<div class="auth-page">
    <div class="auth-card">
        <h1>Create Account</h1>
        {#if error}
            <div class="error">{error}</div>
        {/if}
        <form
            onsubmit={(e) => {
                e.preventDefault();
                handleSignup();
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
            <div class="field">
                <label for="confirm">Confirm Password</label>
                <input
                    type="password"
                    id="confirm"
                    bind:value={confirmPassword}
                    required
                />
            </div>
            <button type="button" class="auth-btn" onclick={handleSignup}
                >Sign Up</button
            >
        </form>
        <p class="switch">
            Already have an account? <a href="/login">Login</a>
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
