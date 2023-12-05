<script>
    export let id;
    export let type = 'text';
    export let placeholder = '';
    export let validateFunction;
    export let value;

    let inputElement;
    let errors = [];
    let isFocused = false;

    function handleInput(event) {
        value = event.target.value;
        errors = validateFunction(value);
    }

    function handleFocus() {
        isFocused = true;
    }

    function handleBlur() {
        isFocused = false;
    }

    $: if (inputElement) {
        inputElement.type = type;
    }

    $: errorClass = errors.length > 0 && !isFocused ? 'error' : '';
</script>

<div class="input-group">
    <input 
        bind:this={inputElement}
        id={id}
        name={id}
        placeholder={placeholder}
        bind:value
        on:input={handleInput}
        on:focus={handleFocus}
        on:blur={handleBlur}
        class={errorClass}
    >
    {#if errors.length > 0}
        <div class="popup" aria-live="polite">
            {#each errors as error}
                <div>{error}</div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .input-group {
        position: relative;
        margin-bottom: 20px;
    }

    .popup {
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
        overflow-y: auto;
        max-height: 150px;
        padding: 0.5em;
        display: none;
    }

    .input-group input:focus + .popup:not(:empty) {
        display: block;
    }

    .input-group input {
        width: 100%;
        padding: 0.5em;
        border: 1px solid #ccc;
        border-radius: 4px;
        box-shadow: inset 0 1px 3px rgba(0,0,0,0.1);
    }

    .input-group input:focus {
        border-color: #66afe9;
        outline: none;
        box-shadow: 0 0 8px rgba(102, 175, 233, 0.6);
    }

    .input-group input.error {
        border: 1px solid #d9534f; /* Red border */
    }
</style>
