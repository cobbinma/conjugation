<script lang="ts">
	import Practise from '../../../../components/Practise.svelte';
	import type { ConjugatedVerb } from '../../../../generated/graphql';

	/** @type {import('./$types').PageData} */
	export let data: { verb: ConjugatedVerb | undefined };
</script>

<svelte:head>
	<title>Practise</title>
	<meta name="description" content="practise conjugating verbs" />
</svelte:head>

{#if data.verb}
	<div class="prose h-2/3">
		<Practise verb={data.verb} />
	</div>
{:else}
	<div><h2>verb not found...</h2></div>
{/if}
