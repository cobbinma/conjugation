<script>
	import Header from './Header.svelte';
	import './styles.css';
</script>

<div class="min-h-screen gradient-bg font-sans" data-theme="conjugation">
	<Header />
	<main class="container mx-auto px-4 py-8">
		<slot />
	</main>
</div>
