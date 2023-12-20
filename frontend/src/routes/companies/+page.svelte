<script>
    import CompanyCard from '$lib/components/CompanyCard.svelte';
    export let data;

    const companies = data.companies;
    const error = data.error;

    import { user } from '$lib/stores.js';

    import FormModal from '$lib/components/FormModal.svelte';
    let isFormModalActive = false;
    let currentAction = '';
    let formModalTitle;
    let formAction;
    let formFields = [];

    function openModal(action) {
        currentAction = action;
        isFormModalActive = true;

        if (action === 'create') {
            formModalTitle = 'Create Company';
            formAction = '?/create';
            formFields = [
                { name: 'name', type: 'text', placeholder: 'Name' },
                { name: 'address', type: 'text', placeholder: 'Address' }
            ];
        }
    }

    function handleFormCancel() {
        isFormModalActive = false;
    }
</script>

<div class="components-container">
    {#if error}
        <p class="error">{error}</p>
    {:else}
        <FormModal
            isActive={isFormModalActive} 
            title={formModalTitle}
            formAction={formAction}
            fields={formFields}
            onCancel={handleFormCancel}
        />
        {#if $user && $user.role === 'Admin'}
            <div class="create">
                <button on:click={() => openModal('create')}>Create Company</button>
            </div>
        {/if}
        {#each companies as company}
            {#if $user}
                <CompanyCard 
                    company={company}
                    user={$user}
                />
            {/if}
        {/each}
    {/if}
</div>
