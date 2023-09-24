<script lang="ts">
	import { Tense } from '../generated/graphql';

	let infinitive: string;
	let selected: string | undefined;
	const UNSELECTED = '';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="practise spanish 💃" />
</svelte:head>

<div class="flex justify-center w-full max-w-xs">
	<section>
		<form method="GET" action="/practise/{infinitive}/{selected}">
			<div>
				<input
					placeholder="Type your verb"
					class="input select-primary w-full"
					bind:value={infinitive}
				/>
				<select class="select select-primary w-full mt-1" bind:value={selected}>
					<option disabled selected>Choose a tense</option>
					{#each Object.keys(Tense) as tense}
						<option value={tense}>
							{tense}
						</option>
					{/each}
				</select>
				{#if infinitive !== '' && selected !== UNSELECTED}
					<div><button class="btn btn-primary mt-1">Go</button></div>
				{/if}
			</div>
		</form>
		<div class="mt-5">
			<form method="GET" action="/practise/random">
				<button class="btn btn-secondary">Random</button>
			</form>
		</div>
	</section>
</div>
