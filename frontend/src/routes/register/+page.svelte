<script>
    import { goto } from '$app/navigation';
    import '/src/styles/auth.css';
    import ValidationInput from '/src/lib/components/ValidationInput.svelte';
    import {
        validateUsername,
        validatePassword,
        validateConfirmPassword,
        validateEmail,
        validateRole,
    } from '/src/lib/components/ValidationFunctions.js';

    function toggleToLogin() {
        goto('/login');
    }

    /** @type {import('./$types').ActionData} */
    export let form;

    let username = '';
    let password = '';
    let confirmPassword = '';
    let email = '';
    let role = '';
    $: confirmPasswordValidator = validateConfirmPassword(password);
    
    $: usernameValid = validateUsername(username).length === 0;
    $: passwordValid = validatePassword(password).length === 0;
    $: confirmPasswordValid = confirmPasswordValidator(confirmPassword).length === 0;
    $: emailValid = validateEmail(email).length === 0;
    $: roleValid = validateRole(role).length === 0;
    $: canRegister = usernameValid && passwordValid && confirmPasswordValid && emailValid && roleValid;

    $: showValidationDialog = !canRegister && false;
</script>

<div class="auth-container">
    <h2>REGISTER NOW</h2>
    <form method="POST" action="?/register">
        <ValidationInput 
            id="username"
            type="text"
            placeholder="Username"
            validateFunction={validateUsername}
            bind:value={username}
        />
        <ValidationInput 
            id="password"
            type="password"
            placeholder="Password"
            validateFunction={validatePassword}
            bind:value={password}
        />
        <ValidationInput 
            id="confirmPassword"
            type="password"
            placeholder="Confirm Password"
            validateFunction={confirmPasswordValidator}
            bind:value={confirmPassword}
        />
        <ValidationInput 
            id="email"
            type="email"
            placeholder="Email"
            validateFunction={validateEmail}
            bind:value={email}
        />

        <!-- Role Selection -->
        <div class="input-group">
            <select id="role" name="role" bind:value={role}>
                <option value="">Select Role</option>
                <option value="admin">Admin</option>
                <option value="employer">Employer</option>
                <option value="user">User</option>
            </select>
        </div>

        <!-- Form Submission Error -->
        {#if form?.error}
            <p class="error">{form?.error}</p>
        {/if}

        <!-- Register Button -->
        <div class="register-button-wrapper">
            <button disabled={!canRegister} class="register-button" on:mouseenter={() => showValidationDialog = true} on:mouseleave={() => showValidationDialog = false}>
                REGISTER
            </button>
            {#if showValidationDialog}
                <div class="validation-dialog" aria-live="polite">
                    {#if !usernameValid}<div>Username is invalid.</div>{/if}
                    {#if !passwordValid}<div>Password is invalid.</div>{/if}
                    {#if !confirmPasswordValid}<div>Passwords do not match.</div>{/if}
                    {#if !emailValid}<div>Email is invalid.</div>{/if}
                    {#if !roleValid}<div>Role is not selected properly.</div>{/if}
                </div>
            {/if}
        </div>
    </form>

    <!-- Toggle to Login -->
    <button class="toggle-auth-link" on:click={toggleToLogin}>
        Already have an account? <span>Login</span>
    </button>
</div>

<style>
    button:disabled {
        background-color: #cccccc; /* Gray color */
        color: #666666; /* Darker text color for contrast */
        cursor: not-allowed; /* Shows a 'no action' cursor on hover */
    }

    select {
        width: 100%; /* Full width */
        padding: 10px; /* Some padding */
        margin: 0.1em 0; /* Some margin */
        border: 1px solid #ccc; /* Gray border */
        border-radius: 4px; /* Rounded borders */
        margin-bottom: 1.5em; /* Add some bottom margin */
    }

    option {
        padding: 10px; /* Add some padding for options */
    }

    .register-button-wrapper {
        position: relative;
        margin: 0px;
    }

    .register-button:disabled {
        cursor: not-allowed;
    }

    .validation-dialog {
        box-sizing: border-box;
        position: absolute;
        top: 100%;
        left: 0;
        width: 100%;
        background-color: #f8d7da;
        color: #721c24;
        border: 1px solid #f5c6cb;
        border-radius: 4px;
        text-align: center;
        font-size: 0.9em;
        box-shadow: 0px 2px 5px rgba(0,0,0,0.2);
        z-index: 1000;
        overflow-y: none;
        max-height: 150px;
        padding: 0.5em;
        margin: 0px;
    }
</style>