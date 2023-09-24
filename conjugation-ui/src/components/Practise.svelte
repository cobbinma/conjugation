<script lang="ts">
	import type { ConjugatedVerb } from '../generated/graphql';
	import Game from './Game.svelte';
	import Table from './Table.svelte';

	export let verb: ConjugatedVerb;

	let played: boolean;
	let playing: boolean;
</script>

<div class="text-column">
	<h1>{verb.infinitive}</h1>

	<h2>{verb.tense.toLowerCase()}</h2>

	<p>
		{verb.verbEnglish}
	</p>

	{#if played && !playing}
		<Table {verb} />
	{/if}

	{#if !playing}
		<button class="btn modal-button btn-secondary" on:click={() => (playing = true)}
			>Practise</button
		>
	{:else}
		<div class="modal" class:modal-open={playing}>
			<div class="modal-box">
				<Game
					{verb}
					close={() => {
						playing = false;
						played = true;
					}}
				/>
			</div>
		</div>
	{/if}
</div>
