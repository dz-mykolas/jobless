<script>
    import { onMount } from 'svelte';
    import CompanyCard from '$lib/components/CompanyCard.svelte';

    let companies = [];
    let error = '';

    onMount(async () => {
        try {
            const response = await fetch('https://orange-invention-7676gg766x5fr759-3000.app.github.dev/api/companies');
            if (!response.ok) {
                throw new Error('Failed to fetch');
            }
            companies = await response.json();
        } catch (e) {
            error = e.message;
        }
    });
</script>


<div class="companies-container">
    {#if error}
        <p class="error">{error}</p>
    {:else}
        {#each companies as company}
            <CompanyCard 
                name={company.name} 
                address={company.address} 
                date={company.date}
                url={`/companies/${company.id}/jobs`}
            />
        {/each}
    {/if}
</div>

<style>
    .error {
        color: red;
    }

    .companies-container {
        background-color: #f9f9f9;
        padding: 20px;
        border-radius: 10px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        margin: auto;
        width: 100%;
        max-height: 600px;
        overflow-y: auto;
    }

    @media (min-width: 768px) {
        .companies-container {
            max-width: 750px;
        }
    }
    @media (min-width: 992px) {
        .companies-container {
            max-width: 970px;
        }
    }
    @media (min-width: 1200px) {
        .companies-container {
            max-width: 1170px;
        }
    }
</style>