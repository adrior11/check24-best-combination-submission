const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{astro,html,js,jsx,svelte,ts,tsx,vue}'],
    theme: {
        extend: {
            fontFamily: {
                sans: ['Inter Variable', ...defaultTheme.fontFamily.sans],
            },
            colors: {
                primary: 'var(--color-primary)',
                secondary: 'var(--color-secondary)',
            },
            textColor: {
                default: 'var(--color-text)',
                offset: 'var(--color-text-offset)',
            },
            backgroundColor: {
                default: 'var(--color-background)',
                offset: 'var(--color-background-offset)',
            },
            borderColor: {
                default: 'var(--color-border)',
            },
        },
    },
    plugins: [],
};
