import {
  CFFEMod,
  InstanceDetails,
  MRFEProject,
  Mod
} from "@gd/core_module/bindings"
import { VersionRowTypeData } from "../InfiniteScrollVersionsQueryWrapper"
import { For, Match, Show, Switch, createSignal } from "solid-js"
import { Trans } from "@gd/i18n"
import { Button, Popover, Spinner, Tooltip } from "@gd/ui"
import { format } from "date-fns"
import CopyIcon from "../CopyIcon"
import ModDownloadButton from "../ModDownloadButton"

export interface Props {
  modVersion: VersionRowTypeData
  project: CFFEMod | MRFEProject | undefined
  isCurseforge?: boolean
  instanceId?: number | null
  instanceDetails?: InstanceDetails
  instanceMods?: Mod[]
  installedFile:
    | {
        id: string
        remoteId: string | number
      }
    | undefined
  type: "modpack" | "mod"
}

export interface AdditionalProps {
  loading: boolean
  disabled: boolean
  isInstalled?: boolean
  onPrimaryAction: () => void
}

const CopiableEntity = (props: {
  text: string | undefined | null | number
}) => {
  return (
    <div class="flex items-center text-lightSlate-200 w-60">
      <div class="truncate">
        <Tooltip
          content={<div class="max-w-110 break-all">{props.text || "-"}</div>}
        >
          {props.text || "-"}
        </Tooltip>
      </div>
      <Show when={props.text}>
        <div class="flex-shrink-0 ml-2">
          <CopyIcon text={props.text} />
        </div>
      </Show>
    </div>
  )
}

const RowContainer = (props: Props & AdditionalProps) => {
  const [isHoveringInfoCard, setIsHoveringInfoCard] = createSignal(false)

  return (
    <Switch>
      <Match when={props.modVersion}>
        <div class="py-2 flex flex-col justify-center">
          <h4 class="m-0 pb-2 font-medium text-md">
            {props.modVersion.name.replaceAll(".zip", "")}
          </h4>
          <div class="flex text-sm gap-2 divide-darkSlate-500 text-lightGray-800 divide-x-1">
            <Trans key="explore_versions.tags" />
            <For each={props.modVersion.gameVersions}>
              {(version) => <div>{version}</div>}
            </For>
          </div>
        </div>
        <div class="flex items-center">
          {format(new Date(props.modVersion.datePublished), "dd-MM-yyyy")}
        </div>
        <div class="flex items-center">{props.modVersion.downloads}</div>
        <div
          class="flex items-center"
          classList={{
            "text-green-500":
              props.modVersion.releaseType === "stable" ||
              props.modVersion.releaseType === "release",
            "text-yellow-500": props.modVersion.releaseType === "beta",
            "text-red-500": props.modVersion.releaseType === "alpha"
          }}
        >
          {props.modVersion.releaseType}
        </div>
        <div class="flex items-center">
          <div onClick={(e) => e.stopPropagation()}>
            <Popover
              noPadding
              noTip
              onOpen={() => setIsHoveringInfoCard(true)}
              onClose={() => setIsHoveringInfoCard(false)}
              content={() => (
                <div
                  class="bg-darkSlate-900 rounded-lg shadow-md p-4 text-lightSlate-700 border-darkSlate-700 border-solid border-1 shadow-darkSlate-90 w-110"
                  onClick={(e) => e.stopPropagation()}
                >
                  <div class="text-lightSlate-50 font-bold mb-4 text-xl">
                    <Trans
                      key="addons_versions.technical_info_for"
                      options={{
                        addon_name: props.modVersion.name
                      }}
                    >
                      {""}
                      <span class="italic">{""}</span>
                    </Trans>
                  </div>
                  <div class="flex flex-col w-full">
                    <div class="flex justify-between w-full text-sm">
                      <div class="w-50">
                        <Trans key="addons_versions.project_id" />
                      </div>
                      <CopiableEntity text={props.modVersion.id} />
                    </div>
                    <div class="flex justify-between w-full text-sm">
                      <div class="w-50">
                        <Trans key="addons_versions.file_id" />
                      </div>
                      <CopiableEntity text={props.modVersion.fileId} />
                    </div>
                    <div class="flex justify-between w-full text-sm">
                      <div class="w-50">
                        <Trans key="addons_versions.file_name" />
                      </div>
                      <CopiableEntity text={props.modVersion.fileName} />
                    </div>
                    <div class="flex justify-between w-full text-sm">
                      <div class="w-50">
                        <Trans key="addons_versions.file_size" />
                      </div>
                      <CopiableEntity text={props.modVersion.size} />
                    </div>
                    <div class="flex justify-between w-full text-sm">
                      <div class="w-50">
                        <Trans key="addons_versions.hash" />
                      </div>
                      <CopiableEntity text={props.modVersion.hash} />
                    </div>
                    <div class="flex justify-between w-full text-sm">
                      <div class="w-50">
                        <Trans key="addons_versions.status" />
                      </div>
                      <CopiableEntity text={props.modVersion.status} />
                    </div>
                    <div class="flex justify-between w-full text-sm">
                      <div class="w-50">
                        <Trans key="addons_versions.release_type" />
                      </div>
                      <CopiableEntity text={props.modVersion.releaseType} />
                    </div>
                  </div>
                </div>
              )}
              trigger="click"
              placement="left-end"
              color="bg-darkSlate-900"
            >
              <div
                class="duration-100 ease-in-out cursor-pointer hover:text-lightSlate-50 text-2xl text-lightSlate-700 i-ri:information-fill transition-color"
                classList={{
                  "text-lightSlate-50": isHoveringInfoCard()
                }}
              />
            </Popover>
          </div>
        </div>
        <div class="flex items-center">
          <Switch>
            <Match when={props.type === "mod"}>
              <ModDownloadButton
                size="medium"
                projectId={props.modVersion.id}
                fileId={props.modVersion.fileId}
                isCurseforge={props.isCurseforge || false}
                instanceId={props.instanceId}
                instanceLocked={props.instanceDetails?.modpack?.locked}
                instanceMods={props.instanceMods}
              />
            </Match>
            <Match when={props.type === "modpack"}>
              <Button
                type="primary"
                variant={props.isInstalled ? "green" : undefined}
                rounded={false}
                disabled={props.disabled || props.isInstalled}
                onClick={props.onPrimaryAction}
              >
                <div class="flex gap-2">
                  <Switch>
                    <Match when={props.type === "mod" && !props.instanceId}>
                      <Trans key="rowcontainer.no_instance_selected" />
                    </Match>
                    <Match when={props.loading}>
                      <Trans key="modpack.version_downloading" />
                      <Spinner class="w-5 h-5" />
                    </Match>
                    <Match when={!props.loading && !props.isInstalled}>
                      <Switch>
                        <Match when={props.installedFile}>
                          <Trans key="modpack.version_switch" />
                        </Match>
                        <Match when={!props.installedFile}>
                          <Trans key="modpack.version_download" />
                        </Match>
                      </Switch>
                      <div class="w-5 h-5 i-ri:download-2-fill" />
                    </Match>
                    <Match when={!props.loading && props.isInstalled}>
                      <Trans key="modpack.version_installed" />
                    </Match>
                  </Switch>
                </div>
              </Button>
            </Match>
          </Switch>
        </div>
      </Match>
      <Match when={!props.modVersion}>
        <Trans key="loading" />
      </Match>
    </Switch>
  )
}

export default RowContainer
