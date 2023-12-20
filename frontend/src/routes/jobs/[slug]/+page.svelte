<script>
    import { user } from '$lib/stores.js';

    export let data;
    const applications = data.applications;
    const job = data.job;
    const error = data.error;

    const company_id = 1;

    import ApplicationCard from '$lib/components/ApplicationCard.svelte';
</script>

<div class="components-container">
    {#if error}
        <p class="error">{error}</p>
    {:else}
        {#each applications as application}
            {#if $user}
                <ApplicationCard 
                    application={application}
                    job={job}
                    companyId={company_id}
                    jobId={application.fk_job_id}
                    role={$user.role}
                    user_id={parseInt($user.sub)}
                />
            {/if}
        {/each}
    {/if}
</div>
