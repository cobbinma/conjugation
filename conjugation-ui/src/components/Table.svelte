<script lang="ts">
	import type { Conjugation } from '../generated/graphql';
	import { pronounToString } from '../lib/pronoun';

	export let conjugations: Conjugation[];
</script>

<div class="overflow-x-auto">
	<table class="table-md">
		<!-- head -->
		<thead>
			<tr>
				<th>Pronoun</th>
				<th>Spanish</th>
			</tr>
		</thead>
		<tbody>
			{#each conjugations as conjugation}
				<tr>
					<td>{pronounToString(conjugation.pronoun)}</td>
					<td>{conjugation.spanish}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
