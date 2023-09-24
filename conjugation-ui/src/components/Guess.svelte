<script lang="ts">
	import type { Tense } from '../generated/graphql';

	export let tense: Tense;
	export let pronoun: string;
	export let infinitive: string;
	export let answer: string;
	export let correct: () => void;
	export let done: () => void;

	let guess: string;

	const normalize = (input: string) => input.normalize('NFD').replace(/[\u0300-\u036f]/g, '');

	const check = () => {
		if (normalize(guess).toLowerCase().trim() === normalize(answer).toLowerCase().trim()) {
			correct();
		}

		done();
	};

	function focus(el: HTMLInputElement) {
		el.focus();
	}
</script>

<div>
	<h2>{tense.toString().toLowerCase()}</h2>
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
			<button class="btn btn-secondary">Check</button>
		</form>
	</div>
	<p>Enter your answer above. "{pronoun} ____"</p>
</div>
