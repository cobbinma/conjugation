/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	plugins: [require('@tailwindcss/typography'), require('daisyui')],
	daisyui: {
		themes: [
			{
				conjugation: {
					primary: '#ff6b9d',
					secondary: '#feca57',
					accent: '#48dbfb',
					neutral: '#2d3748',
					'base-100': '#ffffff',
					'base-200': '#f7fafc',
					'base-300': '#edf2f7',
					info: '#54a0ff',
					success: '#1dd1a1',
					warning: '#feca57',
					error: '#ee5a6f'
				}
			}
		]
	},
	theme: {
		extend: {
			fontFamily: {
				sans: ['Poppins', 'system-ui', 'sans-serif'],
				display: ['Quicksand', 'system-ui', 'sans-serif'],
				mono: ['JetBrains Mono', 'monospace']
			},
			animation: {
				'bounce-in': 'bounceIn 0.6s ease-out',
				'slide-up': 'slideUp 0.4s ease-out',
				'fade-in': 'fadeIn 0.3s ease-out',
				shimmer: 'shimmer 2s linear infinite'
			},
			keyframes: {
				bounceIn: {
					'0%': { transform: 'scale(0.3)', opacity: '0' },
					'50%': { transform: 'scale(1.05)' },
					'70%': { transform: 'scale(0.9)' },
					'100%': { transform: 'scale(1)', opacity: '1' }
				},
				slideUp: {
					'0%': { transform: 'translateY(20px)', opacity: '0' },
					'100%': { transform: 'translateY(0)', opacity: '1' }
				},
				fadeIn: {
					'0%': { opacity: '0' },
					'100%': { opacity: '1' }
				},
				shimmer: {
					'0%': { backgroundPosition: '-1000px 0' },
					'100%': { backgroundPosition: '1000px 0' }
				}
			}
		}
	}
};
