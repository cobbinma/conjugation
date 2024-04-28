<script lang="ts">
	import type { Tense, VerbTense } from '../generated/graphql';
	import Practise from './Practise.svelte';
	import Table from './Table.svelte';

	export let tense: VerbTense;
	export let playing: boolean;

	let table: boolean;
</script>

<div class="text-column">
	<h3>{tense.title}</h3>
	<p>{tense.verbEnglish}</p>

	{#if table && !playing}
		<Table conjugations={tense.conjugations} />
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
					{tense}
					close={() => {
						playing = false;
						table = true;
					}}
				/>
			</div>
		</dialog>
	{/if}
</div>
