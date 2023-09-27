<script lang="ts">
	import type { Tense } from '../generated/graphql';

	export let tense: Tense;
	export let pronoun: string;
	export let infinitive: string;
	export let answer: string;
	export let correct: () => void;
	export let finish: () => void;

	let guess: string;

	let marked: boolean;
	let tick: boolean;

	const normalize = (input: string) => input.normalize('NFD').replace(/[\u0300-\u036f]/g, '');

	const check = () => {
		if (marked) {
			finish();
			return;
		}

		if (normalize(guess).toLowerCase().trim() === normalize(answer).toLowerCase().trim()) {
			tick = true;
			correct();
		}

		marked = true;
	};

	function focus(el: HTMLInputElement) {
		el.focus();
	}
</script>

<div>
	<h2>{tense.toString().toLowerCase().replaceAll('_', ' ')}</h2>
	<h3>{pronoun} ({infinitive})</h3>
	<div>
		<form on:submit|preventDefault={check}>
			<input
				type="text"
				placeholder="Type here"
				class="input select-secondary w-full max-w-xs"
				bind:value={guess}
				use:focus
			/>
			{#if !marked}
				<button class="btn btn-secondary">Check</button>
			{:else}
				<button class="btn btn-primary">Next</button>
			{/if}
		</form>
	</div>
	{#if !marked}
		<p>Enter your answer above. "{pronoun} ____"</p>
	{:else if tick}
		<p class="text-green-800">Correct! 🎉</p>
	{:else}
		<p class="text-rose-600">Unlucky, the answer is <i>{answer}</i></p>
	{/if}
</div>
