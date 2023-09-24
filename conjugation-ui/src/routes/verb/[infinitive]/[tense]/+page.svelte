<script lang="ts">
	import Verb from '../../../../components/Verb.svelte';
	import type { ConjugatedVerb } from '../../../../generated/graphql';

	/** @type {import('./$types').PageData} */
	export let data: ConjugatedVerb;
</script>

<svelte:head>
	<title>Practise</title>
	<meta name="description" content="practise conjugating verbs" />
</svelte:head>

<div class="prose h-2/3">
	<Verb verb={data} />
</div>
