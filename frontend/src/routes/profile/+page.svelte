<script>
    import { user } from '$lib/stores.js';

    export let data;
    export let error;
    const applications = data.applications;

    const company_id = 1;

    import ApplicationCard from '../../lib/components/ApplicationCard.svelte';
</script>

<div class="application-container">
    {#if error}
        <p class="error">{error}</p>
    {:else}
        {#each applications as application}
            {#if $user}
                <ApplicationCard 
                    application={application}
                    companyId={company_id}
                    jobId={application.fk_job_id}
                    role={$user.user_role}
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
    
    .application-container {
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
        .application-container {
            max-width: 750px;
        }
    }
    @media (min-width: 992px) {
        .application-container {
            max-width: 970px;
        }
    }
    @media (min-width: 1200px) {
        .application-container {
            max-width: 1170px;
        }
    }
</style>