<script>
    export let company;
    export let role;

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
            formModalTitle = 'Edit Company';
            formAction = '?/edit';
            formFields = [
                { name: 'id', type: 'hidden', value: company.id },
                { name: 'name', type: 'text', placeholder: 'Name', value: company.name },
                { name: 'address', type: 'text', placeholder: 'Address', value: company.address }
            ];
        } else if (action === 'delete') {
            formModalTitle = 'Delete Company';
            formAction = '?/delete';
            formFields = [
                { name: 'id', type: 'hidden', value: company.id },
            ];
        }
    }

    function handleFormCancel() {
        isFormModalActive = false;
    }
</script>

<div class="company-card">
    <a href={`/companies/${company.id}`}>
        <div class="logo-section">
            <img src={logo} alt="Logo" class="company-logo"/>
        </div>
        <div class="info-section">
            <h3 class="company-name">{company.name}</h3>
            <p class="company-address">{company.address}</p>
            <p class="company-description">{description}</p>
        </div>
    </a>
    <div class="button-section">
        {#if role === 'Admin'}
            <FormModal 
                isActive={isFormModalActive} 
                title={formModalTitle}
                formAction={formAction}
                fields={formFields}
                onCancel={handleFormCancel} />
            <button class="edit" on:click={() => openModal('edit')}>Edit</button>
            <button class="remove" on:click={() => openModal('delete')}>Delete</button>
        {/if}
    </div>
</div>

  
<style>
    .company-card {
        display: flex;
        background-color: #fff;
        border-radius: 10px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        overflow: hidden;
        margin-bottom: 20px;
        height: 150px;
    }

    .company-card a {
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

    .company-logo {
        max-width: 80%;
        max-height: 80%;
    }

    .info-section {
        padding: 15px;
        flex-grow: 1;
    }

    .company-name {
        margin: 0;
        color: #333;
        font-size: 1.2em;
    }

    .company-address, .company-description {
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
</style>
