<script>
    export let isActive = false;
    export let title = '';
    export let formAction = '';
    export let fields = [];
    export let onCancel;

    let errorMessage = '';
    let isDisabled = true;

    // Reactive statement to check for empty fields and set error message
    $: {
        isDisabled = fields.some(field => {
            return typeof field.value === 'string' ? !field.value.trim() : !field.value;
        });

        errorMessage = isDisabled ? 'Please fill out all fields.' : '';
    }

    function handleCancel() {
        if (onCancel) {
            onCancel();
        }
    }
</script>

{#if isActive}
    <div class="modal">
        <div class="modal-content">
            <span class="close" on:click={handleCancel}>&times;</span>
            <h2>{title}</h2>
            <form class="input-container" action={formAction} method="POST">
                {#each fields as field (field.name)}
                    {#if field.type === 'text'}
                        <input bind:value={field.value} name={field.name} type="text" placeholder={field.placeholder} />
                    {:else if field.type === 'hidden'}
                        <input bind:value={field.value} name={field.name} type="hidden" />
                    {:else if field.type === 'password'}
                        <input bind:value={field.value} name={field.name} type="password" placeholder={field.placeholder} />
                    {:else if field.type === 'select'}
                        <select bind:value={field.value} name={field.name}>
                            {#each field.options as option}
                                <option value={option.value}>{option.text}</option>
                            {/each}
                        </select>
                    {/if}
                {/each}
                {#if errorMessage}
                    <div class="error-message">{errorMessage}</div>
                {/if}
                <button type="submit" disabled={isDisabled}>Submit</button>
            </form>
        </div>
    </div>
{/if}

<style>
    .modal {
        display: block;
        position: fixed;
        z-index: 1;
        left: 0;
        top: 0;
        width: 100%;
        height: 100%;
        overflow: auto;
        background-color: rgba(0, 0, 0, 0.4);
    }

    .modal h2 {
        margin: 0 0 10px 0;
        font-family: 'Roboto', sans-serif;
    }

    .modal-content {
        background-color: #fefefe;
        margin: 15% auto;
        padding: 20px;
        border: 1px solid #888;
        width: 50%;
        min-width: 300px;
    }

    .input-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        box-sizing: border-box;
    }

    .close {
        color: #aaa;
        float: right;
        font-size: 28px;
        font-weight: bold;
    }

    .close:hover,
    .close:focus {
        color: black;
        text-decoration: none;
        cursor: pointer;
    }

    button {
        width: 100px;
        border: none;
        border-radius: 5px;
        background-color: var(--color-theme-4);
        color: white;
        cursor: pointer;
        font-family: 'Roboto', sans-serif;
        padding: 10px 20px;
        margin: 25px 0 0 0;
    }

    input {
        width: 100%;
        border: 1px solid #ccc;
        border-radius: 5px;
        margin: 10px 0 0 0;
    }

    select {
        width: 50%;
    }
</style>