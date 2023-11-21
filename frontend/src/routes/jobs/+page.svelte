<script>
    import { onMount } from 'svelte';

    let jobs = [];

    onMount(async () => {
        const response = await fetch('https://your-api-endpoint.com/jobs');
        if (response.ok) {
            jobs = await response.json();
        }
    });
</script>
