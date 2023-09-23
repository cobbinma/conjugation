<script lang="ts">
	import { onMount } from 'svelte';
	import { query } from 'svelte-apollo';
	import { gql } from '@apollo/client/core';
	import type { Tense, QueryRootConjugatedVerbArgs, QueryRoot } from '../generated/graphql';
	import Game from './Game.svelte';
	import Table from './Table.svelte';
	import Error from './Error.svelte';

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

{#await $verb}
	<span class="loading loading-spinner loading-lg" />
{:then result}
	{#if result.data?.conjugatedVerb}
		<div class="text-column">
			<h1>{result.data.conjugatedVerb.infinitive}</h1>

			<h2>{result.data.conjugatedVerb.tense.toLowerCase()}</h2>

			<p>
				{result.data.conjugatedVerb.verbEnglish}
			</p>

			{#if played && !playing}
				<Table verb={result.data.conjugatedVerb} />
			{/if}

			{#if !playing}
				<button class="btn modal-button btn-secondary" on:click={() => (playing = true)}
					>Practise</button
				>
			{:else}
				<div class="modal" class:modal-open={playing}>
					<div class="modal-box">
						<Game
							verb={result.data.conjugatedVerb}
							close={() => {
								playing = false;
								played = true;
							}}
						/>
					</div>
				</div>
			{/if}
		</div>
	{:else if result.loading}
		<span class="loading loading-spinner loading-lg" />
	{:else if result.error}
		<Error />
	{/if}
{:catch}
	<Error />
{/await}
