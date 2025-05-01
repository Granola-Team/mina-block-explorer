/** @type {import('tailwindcss').Config} */

module.exports = {
    content: {
        files: ["*.html","../rust/**/*.{html,rs}"],
    },
    theme: {
        fontFamily: {
            sans: ['system-ui', 'sans-serif'],
            mono: ['Menlo', 'monospace']
        },
        extend: {
            colors: {
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
                'green': '#56D05F',
                'green-dark': '#2B9546',
                'blue': '#14DBF6',
                'blue-dark': '#0EACD3',
                'purple': '#8572E6',
                'dark-blue': '#4392F7',
                'card-purple': '#F4F2FF',
                'card-blue': '#ECFBFF',
                'card-green': '#EEFCEF',
                'status-success': '#56D05F',
                'status-failed': '#FB7631',
                'status-unknown': '#F8AC4B',
          }
        }

    },
    plugins: [
        require('@tailwindcss/container-queries'),
    ],

}
