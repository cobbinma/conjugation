<script lang="ts">
	import VerbComponent from '../../components/Verb.svelte';
	import type { Verb } from '../../generated/graphql';

	/** @type {import('./$types').PageData} */
	let { data }: { data: { verb: Verb | undefined } } = $props();
</script>

<svelte:head>
	<title>{data.verb ? data.verb.infinitive : 'Practise'} - Conjugación</title>
	<meta name="description" content="practice conjugating verbs" />
</svelte:head>

<div class="max-w-6xl mx-auto py-8">
	{#if data.verb}
		<div class="animate-fade-in">
			<VerbComponent verb={data.verb} />
		</div>
	{:else}
		<div class="card-colorful max-w-md mx-auto text-center animate-bounce-in">
			<div class="text-6xl mb-6">😕</div>
			<h2 class="text-3xl font-display font-bold text-error mb-4">Verb not found</h2>
			<p class="text-lg text-neutral/70 mb-6">
				We couldn't find that verb in our database. Try another one!
			</p>
			<button
				class="btn btn-error btn-lg rounded-full px-8 hover:scale-105 transition-transform"
				onclick={() => {
					window.history.back();
				}}
			>
				← Try Again
			</button>
		</div>
	{/if}
</div>
