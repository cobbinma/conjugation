import axios from 'axios';
import type { PageServerLoad } from './$types';
import type { QueryRootVerbArgs } from '../../generated/graphql';

/** @type {import('./$types').PageServerLoad} */
export const load: PageServerLoad = async ({ url }) => {
	try {
		const infinitiveParam = url.searchParams.get('infinitive')?.trim()?.toLowerCase();
		if (!infinitiveParam) {
			return { verb: undefined };
		}

		const variables: QueryRootVerbArgs = {
			infinitive: infinitiveParam
		};

		const response = await axios.post(
			process.env.PUBLIC_API_ENDPOINT_URL || 'http://0.0.0.0:8080/graphql',
			{
				query: `
query SearchVerb($infinitive: String!) {
	verb(infinitive: $infinitive) {
		infinitive
		infinitiveEnglish
		gerundio
		gerundioEnglish
		tenses {
			title
			tense
			mood
			infinitive
			verbEnglish
			conjugations {
				pronoun
				spanish
			}
		}
	}
}
	`,
				variables
			}
		);

		return { verb: response?.data?.data?.verb };
	} catch (error) {
		console.error(`Error in load function for /: ${error}`);
	}
};
