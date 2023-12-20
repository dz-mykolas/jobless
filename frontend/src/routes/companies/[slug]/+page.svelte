<script>
    import JobCard from '$lib/components/JobCard.svelte';
    export let data;

    const jobs = data.jobs;
    const error = data.error;
    const company = data.company;

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
            formModalTitle = 'Create Job';
            formAction = '?/create';
            formFields = [
                { name: 'companyId', type: 'hidden', value: company.id },
                { name: 'title', type: 'text', placeholder: 'Title' },
                { name: 'description', type: 'text', placeholder: 'Description' }
            ];
        }
    }

    function handleFormCancel() {
        isFormModalActive = false;
    }
</script>

<div class="jobs-container">
    <h4>Jobs in company: {company.name}</h4>
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
        {#if $user && ($user.role === 'Admin' || $user.role === 'Employer')}
            <button class="create" on:click={() => openModal('create')}>Create</button>
        {/if}
        {#each jobs as job}
            {#if $user}
                <JobCard 
                    job={job}
                    role={$user.role}
                    user_id={parseInt($user.sub)}
                />
            {/if}
        {/each}
    {/if}
</div>

<style>
    .error {
        color: red;
    }
    
    .jobs-container {
        background-color: #f9f9f9;
        padding: 20px;
        border-radius: 10px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        margin: auto;
        width: 100%;
        max-height: 600px;
        overflow-y: auto;
        box-sizing: border-box;
    }
    
    @media (min-width: 768px) {
        .jobs-container {
            max-width: 750px;
        }
    }
    @media (min-width: 992px) {
        .jobs-container {
            max-width: 970px;
        }
    }
    @media (min-width: 1200px) {
        .jobs-container {
            max-width: 1170px;
        }
    }
</style>