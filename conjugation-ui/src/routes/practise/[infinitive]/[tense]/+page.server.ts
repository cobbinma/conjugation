import { Tense, type QueryRootSearchVerbArgs } from '../../../../generated/graphql';
import axios from 'axios';

/** @type {import('./$types').PageServerLoad} */
export async function load({ params }) {
	const tense = Tense[params.tense as keyof typeof Tense];

	try {
		const variables: QueryRootSearchVerbArgs = {
			infinitive: params.infinitive?.trim()?.toLowerCase() || '',
			tense
		};

		console.log(process.env.PUBLIC_API_ENDPOINT_URL);

		const response = await axios.post(process.env.PUBLIC_API_ENDPOINT_URL || '', {
			query: `
		query SearchVerb($infinitive: String!, $tense: Tense!) {
			searchVerb(infinitive: $infinitive, tense: $tense) {
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

		return { verb: response?.data?.data?.searchVerb };
	} catch (error) {
		console.error(`Error in load function for /: ${error}`);
	}
}
