<script>
    import { goto } from '$app/navigation';
    
    let email = '';
    let password = '';
    let confirmPassword = '';

    // Function to handle registration (to be implemented)
    async function handleRegister() {
        // Add your logic to POST to the register API endpoint
    }

    function toggleToLogin() {
        goto('/login');
    }
</script>

<div class="auth-container">
    <h2>REGISTER NOW</h2>
    <div class="input-group">
        <input type="email" placeholder="Email" bind:value={email}>
    </div>
    <div class="input-group">
        <input type="password" placeholder="Password" bind:value={password}>
    </div>
    <div class="input-group">
        <input type="password" placeholder="Confirm Password" bind:value={confirmPassword}>
    </div>
    <button on:click={handleRegister}>REGISTER</button>
    <button class="toggle-auth-link" on:click={toggleToLogin}>
        If you already have an account? <span>Login</span>
    </button>
</div>