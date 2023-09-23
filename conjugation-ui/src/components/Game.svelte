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
	<div class="modal">
		<div class="content">
			<div class="close-button" on:click={close}>x</div>
			{#if !yo}
				<Guess
					tense={verb.tense}
					pronoun="Yo"
					infinitive={verb.infinitive}
					answer={verb.yo}
					correct={() => score++}
					done={() => (yo = true)}
				/>
			{:else if !tu}
				<Guess
					tense={verb.tense}
					pronoun="Tu"
					infinitive={verb.infinitive}
					answer={verb.tu}
					correct={() => score++}
					done={() => (tu = true)}
				/>
			{:else if !el}
				<Guess
					tense={verb.tense}
					pronoun="El"
					infinitive={verb.infinitive}
					answer={verb.el}
					correct={() => score++}
					done={() => (el = true)}
				/>
			{:else if !nosotros}
				<Guess
					tense={verb.tense}
					pronoun="Nosotros"
					infinitive={verb.infinitive}
					answer={verb.nosotros}
					correct={() => score++}
					done={() => (nosotros = true)}
				/>
			{:else if !vosotros}
				<Guess
					tense={verb.tense}
					pronoun="Vosotros"
					infinitive={verb.infinitive}
					answer={verb.vosotros}
					correct={() => score++}
					done={() => (vosotros = true)}
				/>
			{:else if !ellos}
				<Guess
					tense={verb.tense}
					pronoun="Ellos"
					infinitive={verb.infinitive}
					answer={verb.ellos}
					correct={() => score++}
					done={() => (ellos = true)}
				/>
			{:else}
				<h3>You scored {+((score / 6) * 100).toFixed(2)}%!</h3>
				You got {score} out of 6 correct.
			{/if}
		</div>
	</div>
</div>

<style>
	.modal {
		position: fixed;
		z-index: 9999;
		left: 0;
		top: 0;
		width: 100%;
		height: 100%;
		overflow: auto;
		background-color: rgb(0, 0, 0);
		background-color: rgba(0, 0, 0, 0.4);
	}
	.content {
		position: relative;
		background-color: #fefefe;
		margin: 15% auto;
		padding: 20px;
		border: 1px solid #888;
		width: 80%;
	}
	.close-button {
		position: absolute;
		top: 0;
		right: 0.5rem;
		color: #aaa;
		font-size: 28px;
		font-weight: bold;
		cursor: pointer;
	}
	.close-button:hover,
	.close-button:focus {
		color: black;
		text-decoration: none;
		cursor: pointer;
	}
</style>
