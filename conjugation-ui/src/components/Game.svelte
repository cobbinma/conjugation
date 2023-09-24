<script lang="ts">
	import type { ConjugatedVerb } from '../generated/graphql';
	import Guess from './Guess.svelte';

	export let verb: ConjugatedVerb;
	export let close: () => void;

	let yo: boolean;
	let tu: boolean;
	let el: boolean;
	let nosotros: boolean;
	let vosotros: boolean;
	let ellos: boolean;

	let score: number = 0;
</script>

<div>
	{#if !yo}
		<Guess
			tense={verb.tense}
			pronoun="Yo"
			infinitive={verb.infinitive}
			answer={verb.yo}
			correct={() => score++}
			finish={() => (yo = true)}
		/>
	{:else if !tu}
		<Guess
			tense={verb.tense}
			pronoun="Tu"
			infinitive={verb.infinitive}
			answer={verb.tu}
			correct={() => score++}
			finish={() => (tu = true)}
		/>
	{:else if !el}
		<Guess
			tense={verb.tense}
			pronoun="El"
			infinitive={verb.infinitive}
			answer={verb.el}
			correct={() => score++}
			finish={() => (el = true)}
		/>
	{:else if !nosotros}
		<Guess
			tense={verb.tense}
			pronoun="Nosotros"
			infinitive={verb.infinitive}
			answer={verb.nosotros}
			correct={() => score++}
			finish={() => (nosotros = true)}
		/>
	{:else if !vosotros}
		<Guess
			tense={verb.tense}
			pronoun="Vosotros"
			infinitive={verb.infinitive}
			answer={verb.vosotros}
			correct={() => score++}
			finish={() => (vosotros = true)}
		/>
	{:else if !ellos}
		<Guess
			tense={verb.tense}
			pronoun="Ellos"
			infinitive={verb.infinitive}
			answer={verb.ellos}
			correct={() => score++}
			finish={() => (ellos = true)}
		/>
	{:else}
		<h3>You scored {+((score / 6) * 100).toFixed(2)}%!</h3>
		You got {score} out of 6 correct.
		<div class="modal-action">
			<!-- 🔵 set false on click -->
			<button class="btn" on:click={close}>Close</button>
		</div>
	{/if}
</div>
