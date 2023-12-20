<script>
    import JobCard from '$lib/components/JobCard.svelte';
    export let data;

    const jobs = data.jobs;
    const error = data.error;

    import { user } from '$lib/stores.js';
</script>

<div class="components-container">
    {#if error}
        <p class="error">{error}</p>
    {:else}
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
