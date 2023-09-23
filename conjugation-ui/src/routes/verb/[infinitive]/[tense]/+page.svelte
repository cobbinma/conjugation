<script lang="ts">
	import { ApolloClient, InMemoryCache } from '@apollo/client/core';
	import { setClient } from 'svelte-apollo';
	import Verb from '../../../../components/Verb.svelte';
	import { page } from '$app/stores';
	import { Tense } from '../../../../generated/graphql';

	const client = new ApolloClient({
		uri: 'http://localhost:8080/graphql',
		cache: new InMemoryCache()
	});
	setClient(client);

	const tense = Tense[$page.data.tense as keyof typeof Tense];
</script>

<div class="text-column">
	<Verb infinitive={$page.data.infinitive} {tense} />
</div>
