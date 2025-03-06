/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  mode: 'jit',
  theme: {
    extend: {},
  },
  plugins: [],
}