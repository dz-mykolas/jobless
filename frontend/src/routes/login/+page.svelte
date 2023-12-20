<script>
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import '/src/styles/auth.css';

    function toggleToRegister() {
        goto('/register');
    }

    /** @type {import('./$types').ActionData} */
    export let form;

    let successMessage = $page.url.searchParams.get('success');
    let errorMessage = $page.url.searchParams.get('error');
</script>

<div class="auth-container">
    <h2>LOGIN NOW</h2>
    <form method="POST" action="?/login">
        <div class="input-group">
            <input name="username" type="text" placeholder="Username">
        </div>
        <div class="input-group">
            <input name="password" type="password" placeholder="Password">
        </div>
        <!-- <div class="options">
            <label class="checkbox-label">
                <input type="checkbox">
                Remember Me
            </label>
            <a href="/" class="forgot-password">Forgot Password?</a>
        </div> -->
        {#if form?.error}<p class="error">{form?.error ?? ''}</p>{/if}
        {#if successMessage}
            <p class="success-message">{successMessage}</p>
        {/if}
        {#if errorMessage}
            <p class="error">{errorMessage}</p>
        {/if}
        <button>LOGIN</button>
    </form>
    <button class="toggle-auth-link" on:click={toggleToRegister}>
        Don't have an account? <span>Register</span>
    </button>      
</div>
