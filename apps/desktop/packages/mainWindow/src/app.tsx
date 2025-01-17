import { createEffect, createSignal, untrack } from "solid-js"
import { useLocation, useRoutes } from "@solidjs/router"
import { routes } from "./route"
import initThemes from "./utils/theme"
import { rspc } from "@/utils/rspcClient"
import { useModal } from "./managers/ModalsManager"
import { useKeyDownEvent } from "@solid-primitives/keyboard"
import { checkForUpdates } from "./utils/updater"
import { windowCloseWarningAcquireLock } from "./managers/ModalsManager/modals/WindowCloseWarning"

interface Props {
  createInvalidateQuery: () => void
}

const App = (props: Props) => {
  const [runItOnce, setRunItOnce] = createSignal(true)
  const Route = useRoutes(routes)
  const modalsContext = useModal()
  const currentRoute = useLocation()

  props.createInvalidateQuery()

  window.onShowWindowCloseModal(() => {
    if (windowCloseWarningAcquireLock) {
      modalsContext?.openModal({
        name: "windowCloseWarning"
      })
    }
  })

  initThemes()

  checkForUpdates()

  const setIsFirstRun = rspc.createMutation(() => ({
    mutationKey: "settings.setSettings"
  }))

  const isFirstRun = rspc.createQuery(() => ({
    queryKey: ["settings.getSettings"]
  }))

  createEffect(() => {
    if (
      isFirstRun.data?.isFirstLaunch &&
      currentRoute.pathname !== "/" &&
      runItOnce()
    ) {
      untrack(() => {
        modalsContext?.openModal({ name: "onBoarding" })
        setIsFirstRun.mutate({
          isFirstLaunch: {
            Set: false
          }
        })
      })
      setRunItOnce(false)
    }
  })

  const event = useKeyDownEvent()

  createEffect(() => {
    // close modal clicking Escape
    const e = event()
    if (e) {
      if (e.key === "Escape") {
        untrack(() => {
          modalsContext?.closeModal()
        })
      }
    }
  })

  return (
    <div class="relative w-screen">
      <div class="z-10 flex h-auto w-screen">
        <main class="max-w-screen relative flex-grow">
          <Route />
        </main>
      </div>
    </div>
  )
}

export default App
