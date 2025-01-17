import {
  Button,
  Checkbox,
  Input,
  Progressbar,
  Skeleton,
  Switch,
  Tooltip
} from "@gd/ui"
import { For, Show, createMemo, createSignal } from "solid-js"
import { Trans, useTransContext } from "@gd/i18n"
import Mod from "./Mod"
import skull from "/assets/images/icons/skull.png"
import { useParams, useRouteData } from "@solidjs/router"
import { rspc } from "@/utils/rspcClient"
import { createStore, produce, reconcile } from "solid-js/store"
import fetchData from "../../instance.data"
import { Mod as Modtype } from "@gd/core_module/bindings"
import { useGDNavigate } from "@/managers/NavigationManager"
import { setLastType } from "@/components/InfiniteScrollModsQueryWrapper"
import { useModal } from "@/managers/ModalsManager"

const Mods = () => {
  const [t] = useTransContext()
  const params = useParams()
  const navigate = useGDNavigate()
  const modalsContext = useModal()

  const [filter, setFilter] = createSignal("")
  const [selectedModsMap, setSelectedModsMap] = createStore<
    Record<string, boolean>
  >({})
  const [isModStatusToggleLoading, setIsModStatusToggleLoading] =
    createSignal(false)
  const routeData: ReturnType<typeof fetchData> = useRouteData()

  const isInstanceLocked = () => routeData.instanceDetails.data?.modpack?.locked

  const deleteModMutation = rspc.createMutation(() => ({
    mutationKey: ["instance.deleteMod"]
  }))
  const disableModMutation = rspc.createMutation(() => ({
    mutationKey: ["instance.disableMod"]
  }))
  const enableModMutation = rspc.createMutation(() => ({
    mutationKey: ["instance.enableMod"]
  }))

  const openFolderMutation = rspc.createMutation(() => ({
    mutationKey: ["instance.openInstanceFolder"]
  }))

  const filteredMods = createMemo(() => {
    const filterName = filter().replaceAll(" ", "").toLowerCase()

    return filter()
      ? routeData.instanceMods?.filter(
          (item) =>
            item.filename
              .toLowerCase()
              .replaceAll(" ", "")
              .includes(filterName) ||
            item.metadata?.name
              ?.toLowerCase()
              .replaceAll(" ", "")
              .includes(filterName) ||
            item.curseforge?.name
              ?.toLowerCase()
              .replaceAll(" ", "")
              .includes(filterName) ||
            item.modrinth?.title
              ?.toLowerCase()
              .replaceAll(" ", "")
              .includes(filterName)
        )
      : routeData.instanceMods
  })

  const selectedMods = createMemo(() => {
    return routeData.instanceMods?.filter((mod) => selectedModsMap[mod.id])
  })

  const updateAllMods = () => {
    modalsContext?.openModal(
      {
        name: "modsUpdater"
      },
      {
        mods: routeData.instanceMods.filter((mod) => mod.has_update),
        instanceId: parseInt(params.id, 10)
      }
    )
  }

  const updateSelectedMods = () => {
    modalsContext?.openModal(
      {
        name: "modsUpdater"
      },
      {
        mods: selectedMods().filter((mod) => mod.has_update),
        instanceId: parseInt(params.id, 10)
      }
    )
  }

  const NoMods = () => {
    return (
      <div class="h-full w-full flex justify-center items-center min-h-90">
        <div class="flex flex-col justify-center items-center text-center">
          <img src={skull} class="w-16 h-16" />
          <p class="text-lightSlate-700 max-w-100">
            <Trans key="instance.no_mods_text" />
          </p>
          <Button
            type="outline"
            size="medium"
            onClick={() => {
              navigate(`/mods?instanceId=${params.id}`)
            }}
          >
            <Trans key="instance.add_mod" />
          </Button>
        </div>
      </div>
    )
  }

  const sortAlphabetically = (a: Modtype, b: Modtype) => {
    if (a.filename < b.filename) return -1
    if (a.filename > b.filename) return 1
    return 0
  }

  const isSelectAllIndeterminate = () => {
    return (
      (selectedMods()?.length || 0) > 0 &&
      selectedMods()?.length !== routeData.instanceMods?.length
    )
  }

  return (
    <div>
      <div
        class="flex items-center justify-between h-16 bg-darkSlate-900 shadow-md duration-100 ease-in-out border-darkSlate-700 border-solid border-1 fixed bottom-4 mx-auto left-1/2 -translate-x-1/2 rounded-md z-50 shadow-darkSlate-900 transition-transform origin-left pr-6 w-130"
        classList={{
          "translate-y-24": selectedMods()?.length === 0
        }}
      >
        <div class="flex items-center h-full">
          <div
            class="flex items-center text-lightSlate-700 hover:text-lightSlate-50 h-full px-6 mr-2"
            onClick={() => setSelectedModsMap(reconcile({}))}
          >
            <div class="text-2xl i-ri:close-fill" />
          </div>
          <div class="text-lightSlate-700">
            <Trans
              key="instance_selected_mods_count"
              options={{
                total: routeData.instanceMods?.length,
                selected: selectedMods()?.length
              }}
            />
          </div>
        </div>
        <div class="flex items-center gap-4">
          <Show when={isInstanceLocked()}>
            <Tooltip
              content={<Trans key="instance.locked_cannot_apply_changes" />}
              placement="top"
              class="max-w-38 text-ellipsis overflow-hidden"
            >
              <Switch
                disabled
                checked={selectedMods()?.every((mod) => mod.enabled) || false}
              />
            </Tooltip>
          </Show>
          <Show when={!isInstanceLocked()}>
            <Switch
              isIndeterminate={
                selectedMods()?.some((mod) => mod.enabled) &&
                selectedMods()?.some((mod) => !mod.enabled)
              }
              isLoading={isModStatusToggleLoading()}
              checked={selectedMods()?.every((mod) => mod.enabled) || false}
              onChange={async (event) => {
                if (isModStatusToggleLoading()) return

                setIsModStatusToggleLoading(true)

                let action = event.target.checked

                if (
                  selectedMods()?.some((mod) => mod.enabled) &&
                  selectedMods()?.some((mod) => !mod.enabled)
                ) {
                  action = true
                }

                const modsThatNeedApply = selectedMods()?.filter(
                  (mod) => mod.enabled !== action
                )

                for (const mod of modsThatNeedApply || []) {
                  if (action) {
                    await enableModMutation.mutateAsync({
                      instance_id: parseInt(params.id, 10),
                      mod_id: mod.id
                    })
                  } else {
                    await disableModMutation.mutateAsync({
                      instance_id: parseInt(params.id, 10),
                      mod_id: mod.id
                    })
                  }

                  await new Promise((resolve) => setTimeout(resolve, 10))
                }

                setIsModStatusToggleLoading(false)
              }}
            />
          </Show>
          <Show when={isInstanceLocked()}>
            <Tooltip
              content={<Trans key="instance.locked_cannot_apply_changes" />}
              placement="top"
              class="max-w-38 text-ellipsis overflow-hidden"
            >
              <div class="flex items-center gap-2 cursor-pointer text-lightSlate-700">
                <span class="text-2xl i-ri:delete-bin-2-fill" />
                <Trans key="instance.delete_mod" />
              </div>
            </Tooltip>
          </Show>
          <Show when={!isInstanceLocked()}>
            <div
              class="flex items-center gap-2 cursor-pointer text-lightSlate-700 hover:text-red-500 duration-100 ease-in-out transition"
              onClick={() => {
                Object.keys(selectedModsMap).forEach((mod) => {
                  deleteModMutation.mutate({
                    instance_id: parseInt(params.id, 10),
                    mod_id: mod
                  })
                })
              }}
            >
              <span class="text-2xl i-ri:delete-bin-2-fill" />
              <Trans key="instance.delete_mod" />
            </div>
          </Show>
          <Show
            when={selectedMods().filter((mod) => mod.has_update).length > 0}
          >
            <Show when={isInstanceLocked()}>
              <Tooltip
                content={<Trans key="instance.locked_cannot_apply_changes" />}
                placement="top"
                class="max-w-38 text-ellipsis overflow-hidden"
              >
                <div class="flex items-center gap-2 text-lightSlate-700">
                  <span class="text-2xl i-ri:download-2-fill" />
                  <Trans key="instance.update_mods" />
                </div>
              </Tooltip>
            </Show>
            <Show when={!isInstanceLocked()}>
              <div
                class="flex items-center gap-2 cursor-pointer text-lightSlate-700 hover:text-green-500 duration-100 ease-in-out transition"
                onClick={() => {
                  updateSelectedMods()
                }}
              >
                <span class="text-2xl i-ri:download-2-fill" />
                <Trans key="instance.update_mods" />
              </div>
            </Show>
          </Show>
        </div>
      </div>

      <div class="flex flex-col duration-100 ease-in-out px-6 bg-darkSlate-800 transition-all z-10 sticky top-14">
        <div class="flex justify-between items-center gap-1 pb-4 flex-wrap">
          <div class="flex items-center gap-4 cursor-pointer">
            <Checkbox
              indeterminate={isSelectAllIndeterminate()}
              checked={
                (selectedMods()?.length || 0) > 0 && !isSelectAllIndeterminate()
              }
              onChange={(checked) => {
                let action = checked

                if (isSelectAllIndeterminate()) {
                  action = true
                }

                setSelectedModsMap(
                  produce((prev) => {
                    for (const mod of routeData.instanceMods || []) {
                      prev[mod.id] = action || undefined!
                    }

                    return prev
                  })
                )
              }}
            />
            <Input
              onInput={(e) => setFilter(e.target.value)}
              placeholder={t("instance.mods.search")}
              icon={<div class="i-ri:search-line" />}
              class="text-lightSlate-700 rounded-full"
            />
          </div>
          <div class="flex items-center gap-4">
            {/* <p class="text-lightSlate-700">
              <Trans key="instance.sort_by" />
            </p>
            <Dropdown
              options={[
                { label: t("instance.sort_by_asc"), key: "asc" },
                { label: t("instance.sort_by_desc"), key: "desc" }
              ]}
              value={"asc"}
              rounded
            /> */}
            {/* <div
              class="flex items-center gap-2 cursor-pointer duration-100 ease-in-out transition hover:text-lightSlate-50 text-lightSlate-700"
              onClick={() => {
                openFolderMutation.mutate({
                  folder: "Mods",
                  instance_id: parseInt(params.id, 10)
                });
              }}
            >
              <span class="text-2xl i-ri:filter-line" />
            </div> */}
            <Show when={isInstanceLocked()}>
              <Tooltip
                content={<Trans key="instance.locked_cannot_apply_changes" />}
                placement="top"
                class="max-w-38 text-ellipsis overflow-hidden"
              >
                <Button disabled type="outline" size="medium">
                  <Trans key="instance.add_mod" />
                </Button>
              </Tooltip>
            </Show>
            <Show when={!isInstanceLocked()}>
              <Button
                disabled={isInstanceLocked()}
                type="outline"
                size="medium"
                onClick={() => {
                  setLastType(null)
                  navigate(`/mods?instanceId=${params.id}`)
                }}
              >
                <Trans key="instance.add_mod" />
              </Button>
            </Show>

            <Show
              when={
                routeData.instanceMods?.filter((mod) => mod.has_update).length >
                0
              }
            >
              <Tooltip
                content={
                  <>
                    <Show when={isInstanceLocked()}>
                      <Trans key="instance.locked_cannot_apply_changes" />
                    </Show>
                    <Show when={!isInstanceLocked()}>
                      <Trans key="instance.update_all_mods" />
                    </Show>
                  </>
                }
                placement="top"
                class="max-w-38 text-ellipsis overflow-hidden"
              >
                <div
                  class="flex items-center gap-2 duration-100 ease-in-out transition hover:text-green-500 text-lightSlate-700"
                  onClick={() => {
                    if (isInstanceLocked()) return

                    updateAllMods()
                  }}
                >
                  <span class="text-2xl i-ri:download-2-fill" />
                  <div
                    classList={{
                      "w-0": isInstanceLocked()
                    }}
                    class="duration-100 transition-width"
                  >
                    <Progressbar percentage={15} />
                  </div>
                </div>
              </Tooltip>
            </Show>

            <Tooltip
              content={<Trans key="instance.open_mods_folder" />}
              placement="top"
              class="max-w-38 text-ellipsis overflow-hidden"
            >
              <div
                class="flex items-center gap-2 cursor-pointer duration-100 ease-in-out transition hover:text-lightSlate-50 text-lightSlate-700"
                onClick={() => {
                  openFolderMutation.mutate({
                    folder: "Mods",
                    instance_id: parseInt(params.id, 10)
                  })
                }}
              >
                <span class="text-2xl i-ri:folder-open-fill" />
              </div>
            </Tooltip>
          </div>
        </div>
      </div>
      <div class="h-full w-full overflow-y-hidden pb-14">
        <Show
          when={
            routeData.instanceMods &&
            routeData.instanceMods?.length > 0 &&
            !routeData.instanceDetails.isLoading
          }
          fallback={<NoMods />}
        >
          <For each={[...(filteredMods() || [])].sort(sortAlphabetically)}>
            {(mod) => (
              <Mod
                mod={mod}
                setSelectedMods={setSelectedModsMap}
                selectMods={selectedModsMap}
                isInstanceLocked={isInstanceLocked()}
              />
            )}
          </For>
        </Show>
        <Show when={routeData.instanceDetails.isLoading}>
          <Skeleton.sidebarInstances />
        </Show>
      </div>
    </div>
  )
}

export default Mods
