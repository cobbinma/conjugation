import { Pronoun } from '../generated/graphql';
import { pronounToString } from './pronoun';

test('adds special character to tú pronoun', () => {
	expect(pronounToString(Pronoun.Tu)).toBe('Tú');
});

test('adds special character to el pronoun', () => {
	expect(pronounToString(Pronoun.El)).toBe('Él');
});
