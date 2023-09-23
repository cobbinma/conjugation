<script lang="ts">
	import type { ConjugatedVerb } from '../generated/graphql';

	export let verb: ConjugatedVerb;
</script>

<div>
	<table>
		<tr>
			<th>Pronoun</th>
			<th>Spanish</th>
		</tr>
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
	</table>
</div>
