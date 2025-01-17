/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
  theme: {
    extend: {
      colors: {
        darkgd: "rgba(29, 32, 40, 1)",
        graygd: "rgba(147, 153, 170, 1)",
        whitegd: "rgba(255, 255, 255, 1)",
        bluegd: {
          400: "rgba(62, 134, 208, 1)",
          500: "rgba(40, 101, 164, 1)",
          600: "rgba(35, 62, 94, 1)"
        }
      },
      boxShadow: {
        mdgd: "0px 0px 12px 0px rgba(40, 101, 164, 1)"
      },
      padding: {
        mdgd: "24px"
      },
      borderRadius: {
        xssgd: "8px",
        xsgd: "12px",
        smgd: "34px"
      },
      fontSize: {
        smgd: "1.25rem",
        mdgd: "3.125rem"
      },
      keyframes: {
        fadeIn: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" }
        }
      },
      animation: {
        fadeIn: "fadeIn 0.2s ease-in-out"
      }
    }
  },
  plugins: []
}
