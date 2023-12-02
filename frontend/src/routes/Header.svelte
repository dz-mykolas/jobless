<header>
	<canvas id="particle-canvas"></canvas>
	<nav>
		<div class="nav-menu-overlay" on:click={toggleMenu} class:open={menuOpen}></div>
		<div class="nav-menu" transition:slide={{ y: 0, duration: 300 }} class:open={menuOpen}>
			<ul>
				<li aria-current={$page.url.pathname === '/' ? 'page' : undefined}>
					<a href="/">Home</a>
				</li>
				<li aria-current={$page.url.pathname === '/about' ? 'page' : undefined}>
					<a href="/about">About</a>
				</li>
				<li aria-current={$page.url.pathname === '/jobs' ? 'page' : undefined}>
					<a href="/jobs">Jobs</a>
				</li>
				<li aria-current={$page.url.pathname === '/companies' ? 'page' : undefined}>
					<a href="/companies">Companies</a>
				</li>
			</ul>
		</div>
		<button class="hamburger" aria-label="Menu" on:click={toggleMenu} class:open={menuOpen}>
			<svg viewBox="0 0 100 80" width="40" height="40">
				<rect class="line top" width="100" height="20"></rect>
				<rect class="line middle" y="30" width="100" height="20"></rect>
				<rect class="line bottom" y="60" width="100" height="20"></rect>
			</svg>
		</button>
		<div class="user-menu" class:open={menuOpen}>
			<!-- REPLACE THIS WITH DYNAMIC MENU: -->
		  	<!-- 1) Display login and register if not logged in -->
			<!-- 2) Else display profile button -->
			<ul>
				<li aria-current={$page.url.pathname === '/login' || $page.url.pathname === '/register' ? 'page' : undefined}>
					<a href="/login">Login</a>
				</li>
			</ul>
		</div>
	</nav>
</header>

<style>
	header {
		display: flex;
		justify-content: center;
	}
	
	#particle-canvas {
		z-index: -1;
		position: absolute;
		height: 100%;
		width: 100%
	}
	
	nav {
		display: flex;
		justify-content: center;
		--background: rgba(255, 255, 255, 1);
	}
	
	ul {
		position: relative;
		padding: 0;
		margin: 0;
		height: 3.75em;
		display: flex;
		justify-content: center;
		align-items: center;
		list-style: none;
		background-size: contain;
	}
	
	li {
		position: relative;
		height: 100%;
		margin: 0 1vw;
	}

	li[aria-current='page']::before {
		content: '';
		position: absolute;
		left: 0;
		bottom: 10px;
		width: 0;
		border-bottom: solid 2px var(--color-theme-4);
		animation: border_anim 0.35s ease-out forwards;
	}

	@keyframes border_anim {
		0%{
			width: 0%;
		}
		100%{
			width: 100%;
		}
	}
	
	nav a {
		display: flex;
		height: 100%;
		align-items: center;
		padding: 0 0.5rem;
		color: var(--color-text-header);
		font-weight: 700;
		font-size: 1rem;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		text-decoration: none;
		transition: color 0.2s linear;
	}

	a:hover {
		color: var(--color-theme-3);
	}
	
	.user-menu {
		position: absolute;
		right: 0;
		padding-right: 2em;
		display: flex;
	}
	
	/* Default styles for large screens */
	.nav-menu {
		display: flex;
		overflow: hidden;
	}

	.hamburger {
		display: none;
	}

	/* Styles for small screens */
	@media screen and (max-width: 768px) {
		.nav-menu-overlay {
			background-color: rgba(0, 0, 0, 0.5); /* Semi-transparent black */
			position: fixed;
			top: 0;
			left: 0;
			width: 100%;
			height: 100%;
			display: none;
			z-index: 99; /* Below the menu but above other content */
		}

		.nav-menu ul {
			height: auto;
			display: flex;
			flex-direction: column;
			align-items: flex-start;
			margin-top: 10em;
		}

		.nav-menu ul li {
			padding: 2em; /* Adjust top and bottom padding as needed */
			margin: 0;
			width: 100%;
		}

		.nav-menu ul li a {
			font-size: 1.5rem;
		}

		.nav-menu-overlay.open {
			display: block;
		}

		header {
			justify-content: left;
		}

		.nav-menu {
			background: var(--color-bg-0);
			width: 250px; /* Width of the menu */
			display: flex;
			flex-direction: column;
			position: fixed;
			top: 0;
			left: -250px; /* Hide menu off-screen */
			height: 100%;
			overflow-y: auto;
			transition: transform 300ms ease;
			transform: translateX(-100%);
			z-index: 100; /* Ensure the menu is above other content */
		}
		
		.nav-menu.open {
			transform: translateX(0%);
			left: 0;
		}

		.hamburger {
			border: none;
			background: transparent;
			display: flex;
			margin: auto;
			z-index: 101;
			position: relative;
			top: 10px;
			left: 10px;
		}

		button.hamburger:hover {
			cursor: pointer;
		}

		.hamburger svg {
			fill: var(--color-text-header);
			transition: transform 0.3s ease;
		}

		.line {
			transition: transform 0.3s ease;
		}

		.line.middle {
			opacity: 1;
			transition: opacity 0.3s ease;
		}

		/* When menuOpen is true, add the 'open' class to the button */
		.hamburger.open .top {
			transform: translateY(-8px) translateX(25px) rotate(45deg);
		}

		.hamburger.open .middle {
			opacity: 0;
			transition: opacity 0.1s ease-out;
		}

		.hamburger.open .bottom {
			transform: translateY(20px) translateX(-30px) rotate(-45deg);
		}

		.user-menu {
			top: 10px;
			right: 10px;
		}
	}

</style>

<script>
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
  
	let menuOpen = false;

	function toggleMenu() {
		menuOpen = !menuOpen;
		if (menuOpen) {
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
		}
	}

	// Animation
	onMount(() => {
		// modified version of random-normal
		function normalPool(o){
			var r = 0;
			do { 
				var a = Math.round(normal({mean:o.mean,dev:o.dev}));
				if(a < o.pool.length && a >= 0) {
					return o.pool[a]
				}
				r++
			} while(r < 100) 
		}
		
		function randomNormal(o) {
			if (o = Object.assign( {mean: 0, dev: 1, pool: []} , o), Array.isArray(o.pool) && o.pool.length > 0) {
				return normalPool(o);
			}
			var r, a, n, e, l = o.mean, t = o.dev;
			do {
				r = (a = 2 * Math.random() - 1) * a + (n = 2 * Math.random() - 1) * n
			} while(r >= 1);
			return e = a * Math.sqrt(-2 * Math.log(r) / r), t * e + l
		}
		
		const NUM_PARTICLES = 8;
		const PARTICLE_SIZE = 20; // View heights
		const SPEED = 250000; // Milliseconds
		const ALPHA_RAND = rand(0.05, 0.15);
		
		let particles = [];
		
		function rand(low, high) {
			return Math.random() * (high - low) + low;
		}
		
		function createParticle(canvas) {
			const colour = {
				r: 206,
				g: randomNormal({ mean: 190, dev: 5 }),
				b: 185,
				a: ALPHA_RAND,
			};
			return {
				x: -2,
				y: -2,
				diameter: Math.max(0, randomNormal({ mean: PARTICLE_SIZE, dev: PARTICLE_SIZE / 2 })),
				duration: randomNormal({ mean: SPEED, dev: SPEED * 0.10 }),
				amplitude: randomNormal({ mean: 16, dev: 2 }),
				offsetY: randomNormal({ mean: 0, dev: 10 }),
				arc: Math.PI * 2,
				startTime: performance.now() - rand(0, SPEED),
				colour: `rgba(${colour.r}, ${colour.g}, ${colour.b}, ${colour.a})`,
			}
		}
		
		function moveParticle(particle, canvas, time) {
			const progress = ((time - particle.startTime) % particle.duration) / particle.duration;
			const color = particle.colour;
			let newColor = color;

			const regex = /rgba\((\d+),\s*([\d.]+),\s*(\d+),\s*([\d.]+)\)/;
			let match = color.match(regex);
			if (match) {
				let red = match[1];
				let green = match[2];
				let blue = match[3];
				let alpha = match[4];
				if (progress > 0.9) {
					if (alpha > 0.01) {
						alpha -= 0.001;
					}
					newColor = `rgba(${red}, ${green}, ${blue}, ${alpha})`;
				} else if (particle.x > progress) {
					alpha = ALPHA_RAND;
					newColor = `rgba(${red}, ${green}, ${blue}, ${alpha})`;
				}
			}
			
			return {
				...particle,
				x: progress - 0.1,
				y: ((Math.sin(progress * particle.arc) * particle.amplitude) + particle.offsetY),
				colour: newColor,
			};
		}
		
		function drawParticle(particle, canvas, ctx) {
			canvas = document.getElementById('particle-canvas');
			const vh = canvas.height / 100;
			
			ctx.fillStyle = particle.colour;
			ctx.beginPath();
			ctx.ellipse(
			particle.x * canvas.width,
			particle.y * vh + (canvas.height / 2),
			particle.diameter * vh,
			particle.diameter * vh,
			0,
			0,
			2 * Math.PI
			);
			ctx.fill();
		}
		
		function draw(time, canvas, ctx) {
			// Move particles
			particles.forEach((particle, index) => {
				particles[index] = moveParticle(particle, canvas, time);
			})
			
			// Clear the canvas
			ctx.clearRect(0, 0, canvas.width, canvas.height);
			
			// Draw the particles
			particles.forEach((particle) => {
				drawParticle(particle, canvas, ctx);
			})
			
			// Schedule next frame
			requestAnimationFrame((time) => draw(time, canvas, ctx));
		}
		
		function initializeCanvas() {
			let canvas = document.getElementById('particle-canvas');
			canvas.width = canvas.offsetWidth * window.devicePixelRatio;
			canvas.height = canvas.offsetHeight * window.devicePixelRatio;
			let ctx = canvas.getContext("2d");
			
			window.addEventListener('resize', () => {
				canvas.width = canvas.offsetWidth * window.devicePixelRatio;
				canvas.height = canvas.offsetHeight * window.devicePixelRatio;
				ctx = canvas.getContext("2d");
			})
			
			return [canvas, ctx];
		}
		
		function startAnimation() {
			if (document.getElementById('particle-canvas') === null) {
				return;
			}
			const [canvas, ctx] = initializeCanvas();
			
			// Create a bunch of particles
			for (let i = 0; i < NUM_PARTICLES; i++) {
				particles.push(createParticle(canvas));
			}
			
			requestAnimationFrame((time) => draw(time, canvas, ctx));
		};
		
		// Start animation when document is loaded
		(function () {
			if (document.readystate !== 'loading') {
				startAnimation();
			} else {
				document.addEventListener('DOMContentLoaded', () => {
					startAnimation();
				})
			}
		}());
	});
</script>