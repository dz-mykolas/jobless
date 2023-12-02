<script>
    import { goto } from '$app/navigation';
    
    let username = '';
    let password = '';

    // Function to handle login (to be implemented)
    async function handleLogin() {
        // Add your logic to POST to the login API endpoint
    }

    function toggleToRegister() {
        goto('/register');
    }
</script>

<div class="auth-container">
    <h2>LOGIN NOW</h2>
    <div class="input-group">
        <input type="text" placeholder="User Name" bind:value={username}>
    </div>
    <div class="input-group">
        <input type="password" placeholder="Password" bind:value={password}>
    </div>
    <!-- <div class="options">
        <label class="checkbox-label">
            <input type="checkbox">
            Remember Me
        </label>
        <a href="#" class="forgot-password">Forgot Password?</a>
    </div> -->
    <button on:click={handleLogin}>LOGIN</button>
    <button class="toggle-auth-link" on:click={toggleToRegister}>
        Don't have an account? <span>Register</span>
    </button>      
</div>

<style>
    @import "src/styles/auth.css";
</style>