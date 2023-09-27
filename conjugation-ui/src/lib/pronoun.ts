import { Pronoun } from '../generated/graphql';

export const pronounToString = (pronoun: Pronoun): string => {
	switch (pronoun) {
		case Pronoun.El:
			return 'Él';
		case Pronoun.Ellos:
			return 'Ellos';
		case Pronoun.Nosotros:
			return 'Nosotros';
		case Pronoun.Tu:
			return 'Tú';
		case Pronoun.Vosotros:
			return 'Vosotros';
		case Pronoun.Yo:
			return 'Yo';
		default:
			throw new Error('Invalid pronoun');
	}
};
