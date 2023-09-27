import axios from 'axios';
import { Tense, type QueryRootVerbArgs } from '../../generated/graphql';

/** @type {import('./$types').PageServerLoad} */
export async function load({ url }) {
	const t = url.searchParams.get('tense');
	const tense = t ? Tense[t as keyof typeof Tense] : undefined;

	try {
		const variables: QueryRootVerbArgs = {
			infinitive: url.searchParams.get('infinitive')?.trim()?.toLowerCase(),
			tenses: tense ? [tense] : undefined
		};

		const response = await axios.post(process.env.PUBLIC_API_ENDPOINT_URL || '', {
			query: `
		query SearchVerb($infinitive: String, $tenses: [Tense!]) {
			verb(infinitive: $infinitive, tenses: $tenses) {
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

		return { verb: response?.data?.data?.verb };
	} catch (error) {
		console.error(`Error in load function for /: ${error}`);
	}
}
