<script lang="ts">
  import { prove_fibonacci } from '$lib/pkg/login_bg';
	import '../lib/pkg/login';
	let username = "";
	let password = "";
	let loginStatus = "";
	let isLoading = false;

	// In-memory database for users
	const users: Record<string, string> = {};

	function sleep(ms: number) {
		return new Promise(resolve => setTimeout(resolve, ms));
	}

	async function handleLogin() {
    if (!username || !password) {
        loginStatus = "Please enter both username and password.";
        return;
    }

    isLoading = true;
    loginStatus = "";

    try {
        // Generate proof in WASM (client-side)
        let proof = prove_fibonacci();
        console.log("Generated proof:", proof.length);

        // Send proof to the backend for verification
        const response = await fetch("http://localhost:3000/verify", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ proof }),
        });

        const data = await response.json();
		console.log(data);
        
        if (data.valid) {
            if (users[username]) {
                if (users[username] === password) {
                    loginStatus = `Welcome back, ${username}!`;
                } else {
                    loginStatus = "Incorrect password.";
                }
            } else {
                users[username] = password;
                loginStatus = `Account created for ${username}. Welcome!`;
            }
        } else {
            loginStatus = "Proof verification failed.";
        }
    } catch (error) {
        console.error("Error verifying proof:", error);
        loginStatus = "Error verifying proof.";
    } finally {
        isLoading = false;
    }
}

</script>

<svelte:head>
	<title>Login</title>
	<meta name="description" content="Login page" />
</svelte:head>

<section>
	<h1>Login</h1>
	<div class="form">
		<label for="username">Username:</label>
		<input id="username" type="text" bind:value={username} placeholder="Enter username" />

		<label for="password">Password:</label>
		<input id="password" type="password" bind:value={password} placeholder="Enter password" />

		<button on:click={handleLogin} disabled={isLoading}>Log In/Register</button>
	</div>

	{#if isLoading}
		<div class="loading-icon">⏳ Processing...</div>
	{/if}

	<div class="status">
		<p>{loginStatus}</p>
	</div>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		height: 100vh;
	}

	h1 {
		margin-bottom: 1rem;
	}

	.form {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 1rem;
		width: 300px;
	}

	.form input {
		width: 100%;
		padding: 0.5rem;
		font-size: 1rem;
		border: 1px solid #ccc;
		border-radius: 4px;
	}

	button {
		padding: 0.5rem 1rem;
		font-size: 1rem;
		color: #fff;
		background-color: #007bff;
		border: none;
		border-radius: 4px;
		cursor: pointer;
	}

	button:hover {
		background-color: #0056b3;
	}

	button:disabled {
		background-color: #ccc;
		cursor: not-allowed;
	}

	.loading-icon {
		margin-top: 1rem;
		font-size: 1rem;
		color: #555;
	}

	.status {
		margin-top: 1rem;
		color: #555;
		font-size: 1rem;
	}
</style>
