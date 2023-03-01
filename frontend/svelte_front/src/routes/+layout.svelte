<script>
	import Header from './Header.svelte';
	import './styles.scss';
</script>

<div id="app" class="darkTheme">
	<div class="content">

		<Header />

		<main>
			<slot />
		</main>
	
	</div>
</div>

