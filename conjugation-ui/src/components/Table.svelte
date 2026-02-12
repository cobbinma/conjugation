<script lang="ts">
	import type { Conjugation } from '../generated/graphql';
	import { pronounToString } from '../lib/pronoun';

	let { conjugations }: { conjugations: Conjugation[] } = $props();
</script>

<div class="overflow-x-auto bg-base-100 rounded-xl p-4 shadow-inner">
	<table class="table">
		<thead>
			<tr class="border-b-2 border-primary/20">
				<th class="text-base font-semibold text-primary">Pronoun</th>
				<th class="text-base font-semibold text-primary">Conjugation</th>
			</tr>
		</thead>
		<tbody>
			{#each conjugations as conjugation}
				<tr class="hover:bg-primary/5 transition-colors">
					<td class="font-medium text-neutral">{pronounToString(conjugation.pronoun)}</td>
					<td class="font-mono text-lg font-semibold text-accent">{conjugation.spanish}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
