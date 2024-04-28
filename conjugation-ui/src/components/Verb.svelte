<script lang="ts">
	import type { Verb } from '../generated/graphql';
	import Tense from './Tense.svelte';

	export let verb: Verb;

	let table: boolean;
	let playing: boolean;
</script>

<div class="text-column">
	<h1>{verb.infinitive}</h1>
	{#if verb.infinitiveEnglish}
		<p>
			{verb.infinitiveEnglish}
		</p>
	{/if}

	<div>
		<table class="table-md">
			<!-- head -->
			<thead>
				<tr>
					<th>Gerundio</th>
					<th></th>
				</tr>
			</thead>
			<tbody>
					<tr>
						<td>{verb.gerundio}</td>
						<td>{verb.gerundioEnglish}</td>
					</tr>
			</tbody>
		</table>
	</div>

	{#each verb.tenses as tense}
		<Tense tense={tense} playing={playing} />
	{/each}
</div>
