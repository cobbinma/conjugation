<script lang="ts">
	import type { VerbTense } from '../generated/graphql';
	import Practise from './Practise.svelte';
	import Table from './Table.svelte';

	let { tense, playing }: { tense: VerbTense; playing: boolean } = $props();

	let table = $state(false);

	const tenseColors = [
		'from-primary/10 to-primary/5 border-primary/30',
		'from-secondary/10 to-secondary/5 border-secondary/30',
		'from-accent/10 to-accent/5 border-accent/30',
		'from-success/10 to-success/5 border-success/30',
		'from-info/10 to-info/5 border-info/30',
		'from-warning/10 to-warning/5 border-warning/30'
	];

	let colorClass = tenseColors[Math.floor(Math.random() * tenseColors.length)];
</script>

<div class="card bg-white rounded-2xl shadow-lg hover:shadow-xl transition-all duration-300 p-6 border-2 bg-gradient-to-br {colorClass}">
	<div class="mb-4">
		<h3 class="text-2xl font-display font-bold text-neutral mb-2">
			{tense.title}
		</h3>
		<p class="text-lg text-neutral/70 italic">
			{tense.verbEnglish}
		</p>
	</div>

	{#if table && !playing}
		<div class="mb-4 animate-slide-up">
			<Table conjugations={tense.conjugations} />
		</div>
	{/if}

	<div class="flex flex-wrap gap-3">
		<button
			class="btn btn-outline rounded-full px-6 hover:scale-105 transition-transform"
			class:btn-primary={table}
			onclick={() => (table = !table)}
		>
			{#if !table}
				👁️ Show table
			{:else}
				🙈 Hide table
			{/if}
		</button>

		{#if !playing}
			<button
				class="btn btn-primary rounded-full px-6 hover:scale-105 transition-transform shadow-lg"
				onclick={() => (playing = true)}
			>
				🎯 Practice
			</button>
		{:else}
			<dialog class="modal sm:modal-middle" class:modal-open={playing}>
				<div class="modal-box max-w-2xl rounded-3xl shadow-2xl">
					<form method="dialog">
						<button
							class="btn btn-sm btn-circle btn-ghost absolute right-4 top-4 hover:rotate-90 transition-transform duration-200"
							onclick={() => (playing = false)}>✕</button
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
</div>
