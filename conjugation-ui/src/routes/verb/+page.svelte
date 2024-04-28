<script lang="ts">
	import VerbComponent from '../../components/Verb.svelte';
	import type { Verb } from '../../generated/graphql';

	/** @type {import('./$types').PageData} */
	export let data: { verb: Verb | undefined };
</script>

<svelte:head>
	<title>Practise</title>
	<meta name="description" content="practice conjugating verbs" />
</svelte:head>

{#if data.verb}
	<div class="prose h-2/3">
		<VerbComponent verb={data.verb} />
	</div>
{:else}
	<div><h2>verb not found...</h2></div>
	<button
		class="btn mt-5 btn-outline btn-error"
		on:click={() => {
			window.history.back();
		}}>Try Again</button
	>
{/if}
