import axios from 'axios';

/** @type {import('./$types').PageServerLoad} */
export async function load() {
	try {
		console.log(process.env.PUBLIC_API_ENDPOINT_URL);

		const response = await axios.post(process.env.PUBLIC_API_ENDPOINT_URL || '', {
			query: `
		query RandomVerb {
			randomVerb {
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
	`
		});

		return { verb: response?.data?.data?.randomVerb };
	} catch (error) {
		console.error(`Error in load function for /: ${error}`);
	}
}
