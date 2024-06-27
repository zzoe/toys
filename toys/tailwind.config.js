/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
    theme: {
        extend: {
            gridTemplateColumns: {
                // Simple 14 column grid
                '14': 'repeat(14, minmax(0, 1fr))',
            },
            maxWidth: {
                '222': '55.5rem',
            }
        },
    },
    plugins: [require("daisyui")],
};
