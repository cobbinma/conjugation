<script lang="ts">
	import { Tense } from '../generated/graphql';

	let infinitive: string;
	let selected: string | undefined;
	const UNSELECTED = '';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<div class="flex grow space-y-8 justify-center">
	<section>
		<form method="GET" action="/verb/{infinitive}/{selected}">
			<div>
				<input
					placeholder="Type your verb"
					class="input select-primary w-full max-w-xs"
					bind:value={infinitive}
				/>
				<select class="select select-primary w-full max-w-xs" bind:value={selected}>
					<option disabled selected>Choose a tense</option>
					{#each Object.keys(Tense) as tense}
						<option value={tense}>
							{tense}
						</option>
					{/each}
				</select>
				{#if infinitive !== '' && selected !== UNSELECTED}
					<div><button class="btn btn-primary">Go</button></div>
				{/if}
			</div>
		</form>
	</section>
</div>
