<script>
    import Header from './Header.svelte';
    import '/src/styles/styles.css';

	import { user } from '$lib/stores.js';
	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	export let data;

	onMount(() => {
		console.log('data', data);
		user.set(data.user);
	});

	$: if (browser) {
		const pathname = $page.url;
		console.log('pathname', pathname);
		user.set(data.user);
	}
</script>

<div class="app">
	<Header />
	<main>
		<slot />
	</main>

	<footer>
		<p>Visit <a href="https://github.com/dz-mykolas/jobless">Github Page</a> to learn more about this project</p>
	</footer>
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		box-sizing: border-box;
		margin: 0;
		height: 100vh;
	}

	main {
		width: 100%;
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 0px;
		margin: 0px;
		box-sizing: border-box;
	}

	footer {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 12px;
		margin: 0;
	}

	footer a {
		font-weight: bold;
	}

	@media (min-width: 480px) {
		footer {
			padding: 12px 0;
		}
	}
</style>
