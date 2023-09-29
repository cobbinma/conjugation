<script lang="ts">
	import { navigating } from '$app/stores';
	import { Tense } from '../generated/graphql';

	let infinitive: string | undefined;
	let selected: string | undefined;
	const UNSELECTED = '';
	const UNSELECTED_TENSE = 'Choose a tense';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="practise spanish 💃" />
</svelte:head>

<div class="flex justify-center w-full max-w-xs">
	<section>
		<form method="GET" action="/verb">
			{#if infinitive}
				<input type="hidden" name="infinitive" value={infinitive} />
			{/if}
			{#if selected && selected !== UNSELECTED_TENSE}
				<input type="hidden" name="tense" value={selected} />
			{/if}
			<div>
				<input
					placeholder="Type your verb"
					class="input select-primary w-full"
					bind:value={infinitive}
				/>
				<select class="select select-primary w-full mt-1" bind:value={selected}>
					<option disabled selected>{UNSELECTED_TENSE}</option>
					{#each Object.keys(Tense) as tense}
						<option value={tense}>
							{tense.replace(/([A-Z][a-z])/g, ' $1')}
						</option>
					{/each}
				</select>
				{#if infinitive !== '' && selected !== UNSELECTED}
					<div><button class="btn btn-primary mt-1">Go</button></div>
				{/if}
			</div>
		</form>
		<div class="mt-5">
			<form method="GET" action="/verb">
				<button class="btn btn-secondary">Random</button>
			</form>
		</div>
	</section>
	{#if $navigating}
		<span class="loading loading-ball loading-lg" />
	{/if}
</div>
