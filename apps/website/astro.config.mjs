import { defineConfig } from "astro/config"
import mdx from "@astrojs/mdx"
import tailwind from "@astrojs/tailwind"
import sitemap from "@astrojs/sitemap"
import cloudflare from "@astrojs/cloudflare"
import solidJs from "@astrojs/solid-js"

import yaml from "js-yaml"

const response = await Promise.all([
  fetch("https://cdn-raw.gdl.gg/launcher/latest.yml"),
  fetch("https://cdn-raw.gdl.gg/launcher/latest-mac.yml"),
  fetch("https://cdn-raw.gdl.gg/launcher/latest-linux.yml")
])
const yamlfiles = await Promise.all(response.map((val) => val.text()))
const downloadLinks = yamlfiles
  .map((val) => yaml.load(val))
  .map(
    (val) =>
      `https://cdn-raw.gdl.gg/launcher/${val.path.includes("zip") ? val.path.replace("zip", "dmg") : val.path}`
  )

const excludedPages = [
  "user-account-confirmed/",
  "user-deletion-confirmed/",
  "newsletter/confirm/"
]

// https://astro.build/config
export default defineConfig({
  output: "hybrid",
  site: "https://gdlauncher.com",
  adapter: cloudflare(),
  integrations: [
    tailwind(),
    mdx(),
    sitemap({
      filter: (page) =>
        !excludedPages.find(
          (excludedPage) => `https://gdlauncher.com/${excludedPage}` === page
        )
    }),
    solidJs()
  ],
  redirects: {
    "/download/windows": downloadLinks[0],
    "/download/mac": downloadLinks[1],
    "/download/linux": downloadLinks[2]
  }
})
