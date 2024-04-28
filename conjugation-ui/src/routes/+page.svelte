<script lang="ts">
	import { navigating } from '$app/stores';

	let infinitive: string | undefined;
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="practise spanish 💃" />
</svelte:head>

<div class="flex justify-center w-full max-w-xs">
	{#if $navigating}
		<span class="loading loading-ball loading-lg" />
	{:else}
		<section>
			<form method="GET" action="/verb">
				{#if infinitive}
					<input type="hidden" name="infinitive" value={infinitive} />
				{/if}
				<div>
					<input
						placeholder="Type your verb"
						class="input select-primary w-full"
						bind:value={infinitive}
					/>
					{#if infinitive !== ''}
						<div><button class="btn btn-primary mt-1">Go</button></div>
					{/if}
				</div>
			</form>
		</section>
	{/if}
</div>
