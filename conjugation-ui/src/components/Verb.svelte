<script lang="ts">
	import type { Verb } from '../generated/graphql';
	import Tense from './Tense.svelte';

	let { verb }: { verb: Verb } = $props();

	let playing = $state(false);
</script>

<div class="space-y-8">
	<!-- Verb Header -->
	<div class="card-colorful text-center">
		<h1 class="text-5xl md:text-6xl font-display font-bold gradient-text mb-3">
			{verb.infinitive}
		</h1>
		{#if verb.infinitiveEnglish}
			<p class="text-2xl text-neutral/70 font-medium">
				{verb.infinitiveEnglish}
			</p>
		{/if}
	</div>

	<!-- Gerundio Card -->
	<div class="card-colorful bg-gradient-to-br from-accent/5 to-accent/10">
		<h3 class="text-xl font-semibold text-accent mb-4 flex items-center gap-2">
			<span class="text-2xl">💫</span>
			Gerundio
		</h3>
		<div class="grid md:grid-cols-2 gap-4">
			<div class="bg-white rounded-xl p-4 shadow-sm">
				<span class="text-sm text-neutral/60 block mb-1">Spanish</span>
				<span class="text-2xl font-mono font-semibold text-accent">{verb.gerundio}</span>
			</div>
			<div class="bg-white rounded-xl p-4 shadow-sm">
				<span class="text-sm text-neutral/60 block mb-1">English</span>
				<span class="text-2xl font-medium text-neutral">{verb.gerundioEnglish}</span>
			</div>
		</div>
	</div>

	<!-- Tenses Grid -->
	<div class="space-y-6">
		{#each verb.tenses as tense, index}
			<div class="animate-slide-up" style="animation-delay: {index * 0.05}s">
				<Tense {tense} {playing} />
			</div>
		{/each}
	</div>
</div>
