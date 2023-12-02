<script>
    import { onMount } from 'svelte';

    let jobs = [];

    onMount(async () => {
        const response = await fetch('http://localhost:3000/api/jobs');
        if (response.ok) {
            jobs = await response.json();
        }
    });
</script>
