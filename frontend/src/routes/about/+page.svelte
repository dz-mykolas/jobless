<svelte:head>
	<title>About</title>
	<meta name="description" content="About this app" />
</svelte:head>

<div class="text-column">
	<h1>About this app</h1>

	<p>
		This app is a job board for jobless people. It is built with SvelteKit and Rust.
	</p>

	<h3>For backend</h3>
	<pre>cargo run /workspaces/jobless/backend</pre>
	<h3>For frontend</h3>
	<pre>npm run dev /workspaces/jobless/frontend</pre>

	<p>
		Postgres is used for the database. You can set up your own database and configure the connection
		string in <code>backend/.env</code>.
	</p>
</div>
