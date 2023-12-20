<script>
    import JobCard from '$lib/components/JobCard.svelte';
    export let data;

    const jobs = data.jobs;
    const error = data.error;
    const company = data.company;

    import { user } from '$lib/stores.js';
    import { onMount } from 'svelte';

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

    $: assigned_user = null;

    async function fetchUser(id) {
        let response = await fetch(`/users/${id}`);
        let user = await response.json();

        return user;
    }

    onMount(async () => {
        if ($user && $user.role === 'Employer') {
            assigned_user = await fetchUser($user.sub);
            console.log(assigned_user.fk_company_id);
        }
    });
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
        {#if $user && ($user.role === 'Admin' || (assigned_user && ($user.role === 'Employer' && company.id === assigned_user.fk_company_id)))}
            <div class="create">
                <button on:click={() => openModal('create')}>Create Job</button>
            </div>
        {/if}
        <h4>Jobs in company: {company.name}</h4>
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
