/** @type {import('tailwindcss').Config} */

module.exports = {
    content: {
        files: ["*.html","./src/**/*.{html,rs}"],
    },
    theme: {
        fontFamily: {
            sans: ['system-ui', 'sans-serif'],
        },
        colors: {
            ...require('tailwindcss/colors'),
            'granola-orange': "#E39844",
            'light-granola-orange': "#FBF3EA",
            'main-background': "#21252D",
            'secondary-background': "#F8F8F8",
            'table-section': "#FFFFFF",
            'background': '#F9F9F8',
            'table-header-fill': '#EEEDF7',
            'table-row-fill': '#FFFFFF',
            'table-row-text-color': '#25213B',
            'table-header-text-color': '#25213B',
            'pill-green': '#56D05F',
            'pill-blue': '#14DBF6',
            'card-purple': '#F4F2FF',
            'card-blue': '#ECFBFF',
            'card-green': '#EEFCEF',
            'card-text-purple': '#8572E6',
            'card-text-blue': '#25B4D6'
          },
        extend: {},
    },
    plugins: [
        require('@tailwindcss/container-queries'),
    ],
    
}