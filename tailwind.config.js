/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs", "./input.css"],
  },
  theme: {
    extend: {
      fontFamily: {
        'robotomono': ['Roboto Mono', 'monospace'],
      }
    },
  },
  plugins: [
    require('preline/plugin'),
    require("@tailwindcss/typography")
  ],
  darkMode: 'class',
}
