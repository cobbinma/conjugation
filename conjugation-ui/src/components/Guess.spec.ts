import Guess from './Guess.svelte';
import { render, screen, fireEvent } from '@testing-library/svelte';
import '@testing-library/jest-dom';

describe('Guess Component', () => {
	test('should render the component', () => {
		render(Guess, {
			infinitive: 'hablar',
			english: '',
			tense: 'Presente de Indicativo',
			pronoun: 'Yo',
			answer: 'hablo',
			correct: () => {},
			finish: () => {}
		});

		const title = screen.getByText('Presente de Indicativo');
		expect(title).toBeInTheDocument();

		const subtitle = screen.getByText('Yo (hablar)');
		expect(subtitle).toBeInTheDocument();
	});

	test('should correctly guess', async () => {
		let tick: boolean = false;
		let done: boolean = false;
		render(Guess, {
			infinitive: 'hablar',
			tense: 'Presente de Indicativo',
			pronoun: 'Yo',
			answer: 'hablo',
			correct: () => {
				tick = true;
			},
			finish: () => {
				done = true;
			}
		});
		const input = screen.getByRole('textbox') as HTMLInputElement;
		await fireEvent.input(input, { target: { value: 'hablo' } });
		expect(input.value).toBe('hablo');

		let button = screen.getByRole('button');
		expect(button).toHaveTextContent('Check');

		await fireEvent.click(button);

		const feedback = screen.getByText('Correct! 🎉');
		expect(feedback).toBeInTheDocument();

		button = screen.getByRole('button');
		expect(button).toHaveTextContent('Next');

		await fireEvent.click(button);

		expect(tick).toBeTruthy();
		expect(done).toBeTruthy();
	});

	test('should incorrectly guess', async () => {
		let tick: boolean = false;
		let done: boolean = false;
		render(Guess, {
			infinitive: 'hablar',
			tense: 'Presente de Indicativo',
			pronoun: 'Yo',
			answer: 'hablo',
			correct: () => {
				tick = true;
			},
			finish: () => {
				done = true;
			}
		});
		const input = screen.getByRole('textbox') as HTMLInputElement;
		await fireEvent.input(input, { target: { value: 'incorrect' } });
		expect(input.value).toBe('incorrect');

		let button = screen.getByRole('button');
		expect(button).toHaveTextContent('Check');

		await fireEvent.click(button);

		const feedback = screen.getByText('Unlucky, the answer is');
		expect(feedback).toBeInTheDocument();

		button = screen.getByRole('button');
		expect(button).toHaveTextContent('Next');

		await fireEvent.click(button);

		expect(tick).toBeFalsy();
		expect(done).toBeTruthy();
	});
});
