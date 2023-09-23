<script>
	import Header from './Header.svelte';
	import './styles.css';
</script>

<div class="font-mono">
	<Header />
	<div class="h-screen flex items-center justify-center">
		<div class="h-3/4">
			<main>
				<slot />
			</main>
		</div>
	</div>
</div>
