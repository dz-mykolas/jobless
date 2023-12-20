<script>
    export let application;
    export let job;
    export let role;
    export let user_id;
    export let companyId;

    import logo from '$lib/images/placeholder-logo.jpg';

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
            formModalTitle = 'Edit Application';
            formAction = '?/edit';
            formFields = [
                { name: 'id', type: 'hidden', value: application.id },
                { name: 'name', type: 'text', placeholder: 'Name', value: application.name },
                { name: 'description', type: 'text', placeholder: 'Description', value: application.address }
            ];
        } else if (action === 'delete') {
            formModalTitle = 'Delete Application';
            formAction = '?/delete';
            formFields = [
                { name: 'id', type: 'hidden', value: application.id },
                { name: 'companyId', type: 'hidden', value: companyId },
                { name: 'jobId', type: 'hidden', value: application.fk_job_id }
            ];
        } else if (action === 'reject') {
            formModalTitle = 'Reject Application';
            formAction = '?/reject';
            formFields = [
                { name: 'applicationId', type: 'hidden', value: application.id },
                { name: 'companyId', type: 'hidden', value: companyId },
                { name: 'jobId', type: 'hidden', value: application.fk_job_id }
            ];
        } else if (action === 'accept') {
            formModalTitle = 'Accept Application';
            formAction = '?/accept';
            formFields = [
                { name: 'applicationId', type: 'hidden', value: application.id },
                { name: 'companyId', type: 'hidden', value: companyId },
                { name: 'jobId', type: 'hidden', value: application.fk_job_id }
            ];
        }
    }

    function handleFormCancel() {
        isFormModalActive = false;
    }
</script>

<div class="application-card">
    <div class="logo-section">
        {#if application.status === 'Denied'}
            <div class="cross-out">&#10060;</div>
        {:else if application.status === 'Accepted'}
            <div class="check-mark">&#10004;</div>
        {:else}
            <img src={logo} alt="Logo" class="application-logo"/>
        {/if}
    </div>
    <div class="info-section">
        <h3 class="application-name">{application.name}</h3>
        <p class="application-description">{application.description}</p>
    </div>
    <div class="button-section">
        <FormModal 
            isActive={isFormModalActive} 
            title={formModalTitle}
            formAction={formAction}
            fields={formFields}
            onCancel={handleFormCancel} 
        />
        {#if application.status === 'NotDeniedNotAccepted'}
            {#if role === 'Employer' && job.fk_user_id === user_id}
                <button class="accept" on:click={() => openModal('accept')}>Accept</button>
                <button class="reject" on:click={() => openModal('reject')}>Reject</button>
            {/if}
        {/if}
        {#if user_id === application.fk_user_id}
            {#if application.status === 'NotDeniedNotAccepted'}
                <button class="edit" on:click={() => openModal('edit')}>Edit</button>
            {/if}
            <button class="remove" on:click={() => openModal('delete')}>Delete</button>
        {/if}
    </div>
</div>

  
<style>
    .application-card {
        display: flex;
        background-color: #fff;
        border-radius: 10px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        overflow: hidden;
        margin-bottom: 20px;
        height: 150px;
    }

    .logo-section {
        flex: 0 0 100px;
        background-color: #eee;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .application-logo {
        max-width: 80%;
        max-height: 80%;
    }

    .info-section {
        padding: 15px;
        flex-grow: 1;
    }

    .application-name {
        margin: 0;
        color: #333;
        font-size: 1.2em;
    }

    .application-description {
        margin: 5px 0;
        color: #666;
        font-size: 0.9em;
    }

    .button-section {
        display: flex;
        flex-direction: column;
        margin-left: auto;
        margin-right: 30px;
        justify-content: center;
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
        width: 100%;
        cursor: pointer;
        margin: auto;
    }

    .remove {
        background-color: #f44336;
    }

    .cross-out {
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        color: red;
        font-size: 3em;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .check-mark {
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        color: green;
        font-size: 3em;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .accept {
        background-color: #4CAF50;
    }

    .reject {
        background-color: #f44336;
    }
</style>
