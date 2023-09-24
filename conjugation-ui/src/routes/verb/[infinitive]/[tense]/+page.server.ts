import { Tense, type QueryRootConjugatedVerbArgs } from '../../../../generated/graphql';
import axios from 'axios';

/** @type {import('./$types').PageServerLoad} */
export async function load({ params }) {
	const tense = Tense[params.tense as keyof typeof Tense];

	try {
		const variables: QueryRootConjugatedVerbArgs = {
			infinitive: params.infinitive,
			tense
		};

		const response = await axios.post('http://api.process.spanish.internal:8080/graphql', {
			query: `
		query GetVerb($infinitive: String!, $tense: Tense!) {
			conjugatedVerb(infinitive: $infinitive, tense: $tense) {
				tense
				infinitive
				verbEnglish
				yo
				tu
				el
				nosotros
				vosotros
				ellos
			}
		}
	`,
			variables
		});

		return response.data.data.conjugatedVerb;
	} catch (error) {
		console.error(`Error in load function for /: ${error}`);
	}
}
