<script lang="ts">
	import type { ConjugatedVerb } from '../generated/graphql';
	import { pronounToString } from '../lib/pronoun';
	import Guess from './Guess.svelte';

	export let verb: ConjugatedVerb;
	export let close: () => void;

	let score: number = 0;
	let index: number = 0;
	let high: number = verb.conjugations.length;
</script>

<div>
	{#if index < verb.conjugations.length}
		{#each verb.conjugations as conjugation, i}
			{#if index === i}
				<Guess
					tense={verb.tense}
					pronoun={pronounToString(conjugation.pronoun)}
					infinitive={verb.infinitive}
					answer={conjugation.spanish}
					correct={() => score++}
					finish={() => index++}
				/>
			{/if}
		{/each}
	{:else}
		<h3>You scored {+((score / high) * 100).toFixed(2)}%!</h3>
		You got {score} out of {high} correct.
		<div class="modal-action">
			<button class="btn" on:click={close}>Close</button>
		</div>
	{/if}
</div>
