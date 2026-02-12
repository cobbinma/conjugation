<script lang="ts">
	let {
		tense,
		english,
		pronoun,
		infinitive,
		answer,
		current = 1,
		total = 1,
		correct,
		finish
	}: {
		tense: string;
		english: string;
		pronoun: string;
		infinitive: string;
		answer: string;
		current?: number;
		total?: number;
		correct: () => void;
		finish: () => void;
	} = $props();

	let guess = $state('');
	let marked = $state(false);
	let tick = $state(false);

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

<div class="space-y-6">
	<!-- Progress indicator -->
	<div class="flex justify-between items-center mb-4">
		<div class="badge badge-lg bg-primary/20 text-primary border-0 px-4 py-3">
			Question {current} of {total}
		</div>
		<div class="text-3xl">🎯</div>
	</div>

	<!-- Question -->
	<div class="bg-gradient-to-br from-primary/5 to-secondary/5 rounded-2xl p-6 space-y-2">
		<h2 class="text-2xl font-display font-bold text-primary">{tense}</h2>
		<p class="text-lg text-neutral/70 italic">{english}</p>
		<h3 class="text-3xl font-bold text-neutral mt-4">
			{pronoun} <span class="font-mono text-accent">({infinitive})</span>
		</h3>
	</div>

	<!-- Input form -->
	<form
		onsubmit={(e) => {
			e.preventDefault();
			check();
		}}
		class="space-y-4"
	>
		<div>
			<input
				type="text"
				placeholder="Type your answer..."
				class="input-playful w-full text-xl"
				bind:value={guess}
				use:focus
				disabled={marked}
			/>
		</div>
		
		{#if !marked}
			<button type="submit" class="btn-gradient w-full text-xl">
				Check Answer ✓
			</button>
		{:else}
			<button type="submit" class="btn btn-secondary btn-lg w-full rounded-full text-xl hover:scale-105 transition-transform">
				Next Question →
			</button>
		{/if}
	</form>

	<!-- Feedback -->
	<div class="mt-6">
		{#if !marked}
			<div class="bg-info/10 rounded-xl p-4 text-center">
				<p class="text-lg text-info">💡 Complete the conjugation: "{pronoun} ____"</p>
			</div>
		{:else if tick}
			<div class="bg-success/10 rounded-xl p-6 text-center animate-bounce-in border-2 border-success/30">
				<p class="text-3xl mb-2">🎉</p>
				<p class="text-2xl font-bold text-success">Correct!</p>
				<p class="text-lg text-neutral/70 mt-2">Great job!</p>
			</div>
		{:else}
			<div class="bg-error/10 rounded-xl p-6 text-center animate-bounce-in border-2 border-error/30">
				<p class="text-3xl mb-2">💭</p>
				<p class="text-xl font-semibold text-error mb-3">Not quite right</p>
				<p class="text-lg text-neutral/70">
					The correct answer is: <span class="font-mono text-xl font-bold text-error">{answer}</span>
				</p>
			</div>
		{/if}
	</div>
</div>
