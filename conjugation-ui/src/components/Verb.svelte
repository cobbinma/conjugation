<script lang="ts">
	import { onMount } from 'svelte';
	import { query } from 'svelte-apollo';
	import { gql } from '@apollo/client/core';
	import type { Tense, QueryRootConjugatedVerbArgs, QueryRoot } from '../generated/graphql';
	import Game from './Game.svelte';
	import Table from './Table.svelte';

	export let infinitive: string;
	export let tense: Tense;

	let playing: boolean;
	let played: boolean;

	const VERB_QUERY = gql`
		query GetVerb($infinitive: String!, $tense: Tense!) {
			conjugatedVerb(infinitive: $infinitive, tense: $tense) {
				tense
				infinitive
				verbEnglish
				yo
				tu
				el
				nosotros
				vosotros
				ellos
			}
		}
	`;

	const verb = query<QueryRoot, QueryRootConjugatedVerbArgs>(VERB_QUERY, {
		variables: { infinitive, tense }
	});
</script>

<svelte:head>
	<title>Verb</title>
	<meta name="description" content="About this app" />
</svelte:head>

{#await $verb}
	Loading..
{:then result}
	{#if result.data?.conjugatedVerb}
		<div class="text-column">
			<h1>{result.data.conjugatedVerb.infinitive}</h1>

			<p>
				{result.data.conjugatedVerb.verbEnglish}
			</p>

			{#if played && !playing}
				<Table verb={result.data.conjugatedVerb} />
			{/if}

			{#if !playing}
				<button on:click={() => (playing = true)}>Practise</button>
			{:else}
				<Game
					verb={result.data.conjugatedVerb}
					close={() => {
						playing = false;
						played = true;
					}}
				/>
			{/if}
		</div>
	{:else if result.loading}
		Loading..
	{:else if result.error}
		{result.error}
	{/if}
{:catch error}
	<p class="error">{error}</p>
{/await}
