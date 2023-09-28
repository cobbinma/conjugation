<script lang="ts">
	import type { ConjugatedVerb } from '../generated/graphql';
	import Practise from './Practise.svelte';
	import Table from './Table.svelte';

	export let verb: ConjugatedVerb;

	let table: boolean;
	let playing: boolean;
</script>

<div class="text-column">
	<h1>{verb.infinitive}</h1>
	{#if verb.verbEnglish}
		<p>
			{verb.verbEnglish}
		</p>
	{/if}

	<h2>{verb.tense.toLowerCase().replaceAll('_', ' ')}</h2>

	{#if table && !playing}
		<Table conjugations={verb.conjugations} />
	{/if}

	{#if !playing}
		<button class="btn modal-button btn-primary" on:click={() => (playing = true)}>Practise</button>
	{:else}
		<dialog class="modal sm:modal-middle" class:modal-open={playing}>
			<div class="modal-box">
				<form method="dialog">
					<button
						class="btn btn-sm btn-circle btn-ghost absolute right-1 top-1 p-0"
						on:click={() => (playing = false)}>✕</button
					>
				</form>
				<Practise
					{verb}
					close={() => {
						playing = false;
						table = true;
					}}
				/>
			</div>
		</dialog>
	{/if}
	<div>
		<button class="btn modal-button btn-secondary mt-1" on:click={() => (table = !table)}>
			{#if !table}
				Show table
			{:else}
				Hide table
			{/if}
		</button>
	</div>
</div>
