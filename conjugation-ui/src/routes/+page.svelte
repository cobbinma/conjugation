<script lang="ts">
	import { navigating } from '$app/stores';

	let infinitive = $state<string | undefined>(undefined);
</script>

<svelte:head>
	<title>Conjugación - Learn Spanish Verbs</title>
	<meta name="description" content="practise spanish 💃" />
</svelte:head>

<div class="min-h-[80vh] flex items-center justify-center">
	<div class="w-full max-w-2xl px-4">
		{#if $navigating}
			<div class="flex flex-col items-center gap-6 animate-fade-in">
				<span class="loading loading-ring loading-lg text-primary w-20 h-20"></span>
				<p class="text-xl text-primary font-semibold">Loading your verb...</p>
			</div>
		{:else}
			<div class="text-center animate-slide-up">
				<h1 class="text-6xl md:text-7xl font-display font-bold gradient-text mb-4">
					Learn Spanish
				</h1>
				<p class="text-2xl md:text-3xl text-neutral/70 mb-12 font-medium">
					Master verb conjugations with style 🌟
				</p>

				<div class="card-colorful max-w-xl mx-auto">
					<form method="GET" action="/verb">
						{#if infinitive}
							<input type="hidden" name="infinitive" value={infinitive} />
						{/if}
						<div class="space-y-6">
							<div>
								<label for="verb-input" class="block text-left text-lg font-semibold text-neutral mb-3">
									Enter a Spanish verb
								</label>
								<input
									id="verb-input"
									type="text"
									placeholder="hablar, comer, vivir..."
									class="input-playful w-full"
									bind:value={infinitive}
								/>
							</div>
							{#if infinitive && infinitive.trim() !== ''}
								<div class="animate-bounce-in">
									<button type="submit" class="btn-gradient w-full text-xl">
										Let's Go! 🚀
									</button>
								</div>
							{/if}
						</div>
					</form>
				</div>

				<div class="mt-12 flex flex-wrap justify-center gap-3">
					<div class="badge badge-lg bg-primary/20 text-primary border-0 px-6 py-4 text-base font-medium">
						11,466 verbs
					</div>
					<div class="badge badge-lg bg-secondary/20 text-secondary border-0 px-6 py-4 text-base font-medium">
						All tenses
					</div>
					<div class="badge badge-lg bg-accent/20 text-accent border-0 px-6 py-4 text-base font-medium">
						Practice mode
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>
