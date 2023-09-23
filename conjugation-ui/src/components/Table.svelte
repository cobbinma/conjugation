<script lang="ts">
	import type { ConjugatedVerb } from '../generated/graphql';

	export let verb: ConjugatedVerb;
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
			<tr>
				<td>Yo</td>
				<td>{verb.yo}</td>
			</tr>
			<tr>
				<td>Tu</td>
				<td>{verb.tu}</td>
			</tr>
			<tr>
				<td>El</td>
				<td>{verb.el}</td>
			</tr>
			<tr>
				<td>Nosotros</td>
				<td>{verb.nosotros}</td>
			</tr>
			<tr>
				<td>Vosotros</td>
				<td>{verb.vosotros}</td>
			</tr>
			<tr>
				<td>Ellos</td>
				<td>{verb.ellos}</td>
			</tr>
		</tbody>
	</table>
</div>
