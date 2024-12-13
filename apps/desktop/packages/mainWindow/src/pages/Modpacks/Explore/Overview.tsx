/* eslint-disable solid/no-innerhtml */
import { useRouteData } from "@solidjs/router"
import {
  createEffect,
  createSignal,
  Match,
  Show,
  Suspense,
  Switch
} from "solid-js"
import { Skeleton } from "@gd/ui"
import { marked } from "marked"
import sanitizeHtml from "sanitize-html"
import fetchData from "../modpack.overview"

const Overview = () => {
  const routeData: ReturnType<typeof fetchData> = useRouteData()

  const [parsedDescription, setParsedDescription] = createSignal<string | null>(
    null
  )

  createEffect(async () => {
    if (routeData.modpackDescription?.data?.data) {
      setParsedDescription(
        await marked.parse(
          sanitizeHtml(routeData.modpackDescription?.data?.data || "")
        )
      )
    }
  })

  const Description = () => {
    const cleanHtml = () =>
      sanitizeHtml(routeData.modpackDescription?.data?.data || "", {
        allowedTags: sanitizeHtml.defaults.allowedTags.concat([
          "img",
          "iframe"
        ]),
        allowedAttributes: {
          a: ["href", "name", "target", "class"],
          img: ["src", "width", "height", "class"],
          iframe: ["src", "width", "height", "allowfullscreen"]
        },
        allowedIframeHostnames: [
          "www.youtube.com",
          "i.imgur.com",
          "cdn.ko-fi.com"
        ],
        transformTags: {
          a: sanitizeHtml.simpleTransform("a", { class: "text-blue-500" }),
          img: sanitizeHtml.simpleTransform("img", {
            class: "max-w-full h-auto"
          })
        }
      })

    return (
      <Suspense fallback={<Skeleton.modpackOverviewPage />}>
        <div>
          <Switch fallback={<Skeleton.modpackOverviewPage />}>
            <Match when={routeData.isCurseforge}>
              <div
                class="w-full max-w-full overflow-hidden"
                innerHTML={cleanHtml()}
              />
            </Match>
            <Match when={!routeData.isCurseforge}>
              <Show when={parsedDescription()}>
                <div class="w-full" innerHTML={parsedDescription()!} />
              </Show>
            </Match>
          </Switch>
        </div>
      </Suspense>
    )
  }

  return (
    <Switch fallback={<Skeleton.modpackOverviewPage />}>
      <Match when={!routeData.modpackDescription?.isLoading}>
        <Description />
      </Match>
      <Match when={routeData.modpackDescription?.isLoading}>
        <Skeleton.modpackOverviewPage />
      </Match>
    </Switch>
  )
}

export default Overview
