/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./ssr/*.html", "./ssr/src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        primary: {
          50: "#FFF5FA",
          100: "#FFEBF5",
          200: "#FED2EA",
          300: "#FEB4DB",
          400: "#FD91CB",
          500: "#FC5AB0",
          600: "#E20479",
          700: "#C3036A",
          800: "#A5035A",
          900: "#73023E",
          950: "#50012B",
        },
      },
    },
  },
  plugins: [],
};
