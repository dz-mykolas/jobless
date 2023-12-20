<script>
    export let job;
    export let role;
    export let user_id;

    // Placeholder values
    let date = '2020-01-01'
    import logo from '$lib/images/placeholder-logo.jpg';
    let description = 'Lorem ipsum dolor sit amet...';

    import FormModal from '$lib/components/FormModal.svelte';
    let isFormModalActive = false;
    let currentAction = '';
    let formModalTitle;
    let formAction;
    let formFields = [];

    function openModal(action) {
        currentAction = action;
        isFormModalActive = true;

        if (action === 'edit') {
            formModalTitle = 'Edit Job';
            formAction = '?/edit';
            formFields = [
                { name: 'id', type: 'hidden', value: company.id },
                { name: 'title', type: 'text', placeholder: 'Name', value: job.title },
                { name: 'description', type: 'text', placeholder: 'Address', value: job.description }
            ];
        } else if (action === 'delete') {
            formModalTitle = 'Delete Job';
            formAction = '?/delete';
            formFields = [
                { name: 'id', type: 'hidden', value: job.id },
                { name: 'companyId', type: 'hidden', value: job.fk_company_id}
            ];
        } else if (action === 'apply') {
            formModalTitle = 'Apply to Job';
            formAction = '?/apply';
            formFields = [
                { name: 'jobId', type: 'hidden', value: job.id },
                { name: 'companyId', type: 'hidden', value: job.fk_company_id},
                { name: 'name', type: 'text', placeholder: 'Name'},
                { name: 'description', type: 'text', placeholder: 'Description'},
            ];
        }
    }

    function handleFormCancel() {
        isFormModalActive = false;
    }
</script>

<div class="job-card">
    {#if role !== 'User' && (role === 'Employer' && job.fk_user_id === user_id)}
        <div class="details-wrapper">
            <a href={`/jobs/${job.id}`}>
                <div class="logo-section">
                    <img src={logo} alt="Logo" class="job-logo"/>
                </div>
                <div class="info-section">
                    <h3 class="job-title">{job.title}</h3>
                    <p class="job-description">{description}</p>
                </div> 
            </a>
        </div>
    {:else}
        <div class="details-wrapper">
            <div class="logo-section">
                <img src={logo} alt="Logo" class="job-logo"/>
            </div>
            <div class="info-section">
                <h3 class="job-title">{job.title}</h3>
                <p class="job-description">{description}</p>
            </div>
        </div>
    {/if}
    <div class="button-section">
        <FormModal 
            isActive={isFormModalActive} 
            title={formModalTitle}
            formAction={formAction}
            fields={formFields}
            onCancel={handleFormCancel} />
        {#if user_id !== job.fk_user_id}
            <button class="apply" on:click={() => openModal('apply')}>Apply</button>
        {/if}
        {#if role === 'Admin' || (role === 'Employer' && job.fk_user_id === user_id)}
            <button class="edit" on:click={() => openModal('edit')}>Edit</button>
            <button class="remove" on:click={() => openModal('delete')}>Delete</button>
        {/if}
    </div>
</div>

<style>
    .job-card {
        display: flex;
        background-color: #fff;
        border-radius: 10px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        overflow: hidden;
        margin-bottom: 20px;
        height: 150px;
    }

    .details-wrapper {
        display: flex;
        text-decoration: none;
    }

    .job-card a {
        display: flex;
        text-decoration: none;
    }

    .logo-section {
        flex: 0 0 100px;
        background-color: #eee;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .job-logo {
        max-width: 80%;
        max-height: 80%;
    }

    .info-section {
        padding: 15px;
        flex-grow: 1;
    }

    .job-title {
        margin: 0;
        color: #333;
        font-size: 1.2em;
    }

    .job-description {
        margin: 5px 0;
        color: #666;
        font-size: 0.9em;
    }

    button {
        background-color: #4CAF50;
        border: none;
        color: white;
        padding: 10px 20px;
        text-align: center;
        text-decoration: none;
        display: inline-block;
        font-size: 16px;
        cursor: pointer;
        margin: auto;
    }

    .remove {
        background-color: #f44336;
    }

    .apply {
        background-color: #2196F3;
    }
</style>
