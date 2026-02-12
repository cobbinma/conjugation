<script lang="ts">
	import type { VerbTense } from '../generated/graphql';
	import { pronounToString } from '../lib/pronoun';
	import Guess from './Guess.svelte';

	let { tense, close }: { tense: VerbTense; close: () => void } = $props();

	let score = $state(0);
	let index = $state(0);

	// Using a reactive derived to avoid warning about capturing initial value
	let high = $derived(tense.conjugations.length);
</script>

<div class="py-4">
	{#if index < tense.conjugations.length}
		{#each tense.conjugations as conjugation, i}
			{#if index === i}
				<div class="animate-slide-up">
					<Guess
						tense={tense.title}
						english={tense.verbEnglish || ''}
						pronoun={pronounToString(conjugation.pronoun)}
						infinitive={tense.infinitive}
						answer={conjugation.spanish}
						current={index + 1}
						total={high}
						correct={() => score++}
						finish={() => index++}
					/>
				</div>
			{/if}
		{/each}
	{:else}
		<div class="text-center animate-bounce-in">
			<div class="text-7xl mb-6">
				{#if (score / high) >= 0.9}
					🎉
				{:else if (score / high) >= 0.7}
					😊
				{:else if (score / high) >= 0.5}
					😐
				{:else}
					😅
				{/if}
			</div>
			<h3 class="text-4xl font-display font-bold gradient-text mb-4">
				{+((score / high) * 100).toFixed(0)}%
			</h3>
			<p class="text-xl text-neutral/70 mb-8">
				You got <span class="font-bold text-success">{score}</span> out of <span class="font-bold">{high}</span> correct!
			</p>
			<div class="modal-action justify-center">
				<button class="btn btn-primary btn-lg rounded-full px-12 hover:scale-105 transition-transform shadow-lg" onclick={close}>
					Continue
				</button>
			</div>
		</div>
	{/if}
</div>
