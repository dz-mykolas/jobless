<script>
    export let isActive = false;
    export let title = '';
    export let formAction = ''; // URL to which the form will be submitted
    export let fields = []; // Array of objects representing fields
    export let onCancel;

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
                    {/if}
                {/each}
                <button>Submit</button>
            </form>
        </div>
    </div>
{/if}

<style>
    .input-container {
        display: flex;
        flex-direction: column;
        align-items: left;
    }

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

    .modal-content {
        background-color: #fefefe;
        margin: 15% auto;
        padding: 20px;
        border: 1px solid #888;
        width: 80%;
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
</style>