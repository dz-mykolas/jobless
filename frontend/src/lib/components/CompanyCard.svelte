<script>
    export let company;
    export let user;

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
        } else if (action === 'assign-employer') {
            formModalTitle = 'Assign Employer';
            formAction = '?/assign_employer';
        }
    }

    function handleFormCancel() {
        isFormModalActive = false;
    }

    let users = [];

    async function fetchUsers() {
        const response = await fetch('/users/employers');
        if (response.ok) {
            const users = await response.json();
            console.log('Users fetched', users);
            return users.users;
        } else {
            console.error('Error fetching users');
            throw new Error('Failed to fetch users');
        }
    }

    async function onAssignEmployerClick() {
        try {
            const fetchedUsers = await fetchUsers();
            if (fetchedUsers) {
                console.log('Fetched users', fetchedUsers);
                formFields = [
                    { name: 'companyId', type: 'hidden', value: company.id },
                    { 
                        name: 'employerId', 
                        type: 'select', 
                        options: fetchedUsers.map(user => ({ value: user.fk_user_id, text: user.fk_user_id }))
                    }
                ];
            }
            openModal('assign-employer');
        } catch (error) {
            console.error('Error in onAssignEmployerClick:', error);
        }
    }

    $: assigned_user = null;

    async function fetchUser(id) {
        let response = await fetch(`/users/${id}`);
        let user = await response.json();

        return user;
    }

    import { onMount } from 'svelte';

    onMount(async () => {
        if (user && user.role === 'Employer') {
            assigned_user = await fetchUser(user.sub);
            console.log(assigned_user.fk_company_id);
        }
    });

    $: isWorkingHere = (user && user.role === 'Employer' && assigned_user && assigned_user.fk_company_id === company.id);
</script>

<div class="company-card {isWorkingHere ? 'blue-border' : ''}">
    <a href={`/companies/${company.id}`}>
        <div class="logo-section">
            <img src={logo} alt="Logo" class="company-logo"/>
            {#if isWorkingHere}
                <div class="working-here">Working here</div>
            {/if}
        </div>
        <div class="info-section">
            <h3 class="company-name">{company.name}</h3>
            <p class="company-address">{company.address}</p>
            <p class="company-description">{description}</p>
        </div>
    </a>
    <div class="button-section">
        {#if user.role === 'Admin'}
            <FormModal 
                isActive={isFormModalActive} 
                title={formModalTitle}
                formAction={formAction}
                fields={formFields}
                onCancel={handleFormCancel} />
            <button class="edit" on:click={() => openModal('edit')}>Edit</button>
            <button class="remove" on:click={() => openModal('delete')}>Delete</button>
            <button class="assign-employer" on:click={onAssignEmployerClick}>Assign Employer</button>
        {/if}
    </div>
</div>

  
<style>
    .blue-border {
        border: 2px solid blue;
    }

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
        justify-content: center;
        flex-direction: column;
        align-items: center;
    }

    .working-here {
        margin-top: 10px; /* adjust as needed */
        text-align: center;
        font-family: 'Roboto', sans-serif;
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

    .assign-employer {
        background-color: #2196F3;
    }
</style>
