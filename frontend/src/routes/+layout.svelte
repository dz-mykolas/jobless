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

	let currentTime = new Date().toLocaleTimeString();

	// Update the time every second
	setInterval(() => {
		currentTime = new Date().toLocaleTimeString();
	}, 1000);
</script>

<div class="app">
	<link href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&display=swap" rel="stylesheet">
	<Header />
	<main>
		<slot />
	</main>

	<footer>
		<p>Current time: {currentTime}. Visit <a href="https://github.com/dz-mykolas/jobless">Github Page</a> to learn more about this project</p>
	</footer>
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		box-sizing: border-box;
		height: 100vh;
		width: 100vw;
		justify-content: center;
	}

	main {
		width: 100%;
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 0;
		margin: 0;
		overflow: hidden;
		box-sizing: border-box;
	}

	footer {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 12px;
		margin: 0;
		box-sizing: border-box;
		font-size: 1em;
	}

	footer a {
		font-weight: bold;
		margin: 0;
	}

	@media (min-width: 1024px) {
		footer {
			font-size: 5em;
		}
	}

	@media (min-width: 768px) {
		footer {
			font-size: 1em;
		}
	}

	@media (min-width: 480px) {
		footer {
			padding: 12px 0;
		}
	}
</style>
