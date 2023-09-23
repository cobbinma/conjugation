<script lang="ts">
	import { Tense } from '../generated/graphql';

	let infinitive: string;
	let selected: string | undefined;
	const UNSELECTED = '';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<form method="GET" action="/verb/{infinitive}/{selected}">
		<input bind:value={infinitive} />
		<select bind:value={selected}>
			{#each [UNSELECTED, ...Object.keys(Tense)] as tense}
				<option value={tense}>
					{tense}
				</option>
			{/each}
		</select>
		{#if infinitive !== '' && selected !== UNSELECTED}
			<div><button>Go</button></div>
		{/if}
	</form>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}
</style>
