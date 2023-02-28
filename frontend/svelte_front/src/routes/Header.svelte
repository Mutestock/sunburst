<script>
	import { page } from '$app/stores';
	import ToggleThemeButton from '$lib/ToggleThemeButton.svelte';
</script>

<header>
	<nav>
		<ul>
			<li aria-current={$page.url.pathname === '/' ? 'page' : undefined}>
				<a href="/">Home</a>
			</li>
			<li aria-current={$page.url.pathname === '/about' ? 'page' : undefined}>
				<a href="/about">About</a>
			</li>
		</ul>
	</nav>

	<ToggleThemeButton />
</header>


<style lang=scss>
	header{
		background-color: #343434;
		color: white;
		border: 3px solid black;
	}
</style>