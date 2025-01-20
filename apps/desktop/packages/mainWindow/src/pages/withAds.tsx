import AppNavbar from "@/components/Navbar"
import { Outlet, useRouteData } from "@solidjs/router"
import fetchData from "./app.data"
import { setMappedMcVersions, setMcVersions } from "@/utils/mcVersion"
import {
  setCurseforgeCategories,
  setModrinthCategories,
  setSupportedModloaders
} from "@/utils/sidebar"
import { createEffect } from "solid-js";

function withAdsLayout() {
  const routeData: ReturnType<typeof fetchData> = useRouteData()

  createEffect(() => {
    if (routeData.minecraftVersions.data) {
      setMcVersions(routeData.minecraftVersions.data)
      routeData.minecraftVersions.data.forEach((version) => {
        if (version.type === "release") {
          setMappedMcVersions((prev) => [
            ...prev,
            { label: version.id, key: version.id }
          ])
        }
      })
      setMappedMcVersions((prev) => [
        { key: "", label: "All version" },
        ...prev
      ])
    }
  })

  createEffect(() => {
    if (routeData.curseForgeModloaders.data) {
      setSupportedModloaders("curseforge", routeData.curseForgeModloaders.data)
    }
    if (routeData.modrinthModloaders.data) {
      setSupportedModloaders("modrinth", routeData.modrinthModloaders.data)
    }
  })

  createEffect(() => {
    if (routeData.curseforgeCategories.data)
      setCurseforgeCategories(routeData.curseforgeCategories.data.data)
  })

  createEffect(() => {
    if (routeData.modrinthCategories.data)
      setModrinthCategories(routeData.modrinthCategories.data)
  })

  return (
    <>
      <AppNavbar />
      <div
        class="flex w-screen z-10 h-auto"
        style={{
          background: "var(--ads-sidebar-background)"
        }}
      >
        <main class="relative flex-grow">
          <div class="flex justify-end h-[calc(100vh-60px)]">
            <div
              style={{
                width: `calc(100vw - ${0}px)`
              }}
            >
              <Outlet />
            </div>
            <div class="flex flex-col justify-between h-[calc(100vh-100px)]">
              <div class="flex justify-center">
              </div>
            </div>
          </div>
        </main>
      </div>
    </>
  )
}



