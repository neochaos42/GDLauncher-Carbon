import { Button, Checkbox, Dropdown, Input } from "@gd/ui"
import { For, Show } from "solid-js"
import { Trans, useTransContext } from "@gd/i18n"
import ResourcePack from "./ResourcePack"
import skull from "/assets/images/icons/skull.png"

interface IResourcepack {
  title: string
  enabled: boolean
  mcversion: string
  modloaderVersion: string
  resourcePackVersion: string
}

const resourcePacks: IResourcepack[] = [
  {
    title: "Mods1",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.17"
  },
  {
    title: "Mods2",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.15"
  },
  {
    title: "Mods3",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.15"
  },
  {
    title: "Mods4",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.14"
  },
  {
    title: "Mods5",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.17"
  },
  {
    title: "Mods6",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.17"
  },
  {
    title: "Mods7",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.17"
  },
  {
    title: "Mods8",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.17"
  },
  {
    title: "Mods9",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",

    resourcePackVersion: "1.17"
  },
  {
    title: "Mods8",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.17"
  },
  {
    title: "Mods9",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.17"
  },
  {
    title: "Mods8",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.17"
  },
  {
    title: "Mods9",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.17"
  },
  {
    title: "Mods8",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.17"
  },
  {
    title: "Mods9",
    enabled: true,
    mcversion: "1.19.2",
    modloaderVersion: "2.1.3",
    resourcePackVersion: "1.17"
  }
]

const NoResourcePacks = () => {
  return (
    <div class="h-full min-h-90 w-full flex justify-center items-center">
      <div class="flex flex-col justify-center items-center text-center">
        <img src={skull} class="w-16 h-16" />
        <p class="text-lightSlate-700 max-w-100">
          <Trans
            key="instance.no_resource_packs_text"
            options={{
              defaultValue:
                "At the moment this modpack does not contain resource packs, but you can add packs yourself from your folder"
            }}
          />
        </p>
        <Button type="outline" size="medium">
          <Trans
            key="instance.add_resource_pack"
            options={{
              defaultValue: "+ Add pack"
            }}
          />
        </Button>
      </div>
    </div>
  )
}

const ResourcePacks = () => {
  const [t] = useTransContext()
  return (
    <div>
      <div class="flex flex-col bg-darkSlate-800 z-10 transition-all duration-100 ease-in-out sticky pt-10 top-30">
        <div class="flex justify-between items-center pb-4 flex-wrap gap-1">
          <Input
            placeholder="Type Here"
            icon={<div class="i-ri:search-line" />}
            class="w-full rounded-full text-lightSlate-700"
            inputClass=""
          />
          <div class="flex gap-3 items-center">
            <p class="text-lightSlate-700">
              <Trans
                key="instance.sort_by"
                options={{
                  defaultValue: "Sort by:"
                }}
              />
            </p>
            <Dropdown
              options={[
                { label: t("instance.sort_by_asc"), key: "asc" },
                { label: t("instance.sort_by_desc"), key: "desc" }
              ]}
              value={"asc"}
              rounded
            />
          </div>
          <Button type="outline" size="medium">
            <Trans
              key="instance.add_resource_pack_pack"
              options={{
                defaultValue: "+ Add ResourcePack"
              }}
            />
          </Button>
        </div>
        <div class="flex justify-between text-lightSlate-700 z-10 mb-6">
          <div class="flex gap-4">
            <div class="flex items-center gap-2 cursor-pointer">
              <Checkbox checked={true} disabled={false} />
              <Trans
                key="instance.select_all_resource_pack"
                options={{
                  defaultValue: "Select All"
                }}
              />
            </div>
            <div class="flex items-center gap-2 cursor-pointer hover:text-lightSlate-50 transition duration-100 ease-in-out">
              <span class="i-ri:folder-open-fill text-2xl" />
              <Trans
                key="instance.open_resource_packs_folder"
                options={{
                  defaultValue: "Open folder"
                }}
              />
            </div>
            <div class="flex items-center gap-2 cursor-pointer hover:text-lightSlate-50 transition duration-100 ease-in-out">
              <span class="text-2xl i-ri:forbid-line" />
              <Trans
                key="instance.disable_resource_pack"
                options={{
                  defaultValue: "disable"
                }}
              />
            </div>
            <div class="flex items-center gap-2 cursor-pointer hover:text-lightSlate-50 transition duration-100 ease-in-out">
              <span class="i-ri:delete-bin-2-fill text-2xl" />
              <Trans
                key="instance.delete_resource_pack"
                options={{
                  defaultValue: "delete"
                }}
              />
            </div>
          </div>
          <div>
            {resourcePacks.length}
            <Trans
              key="instance.resource_packs"
              options={{
                defaultValue: "Resource packs"
              }}
            />
          </div>
        </div>
      </div>
      <div class="h-full overflow-y-hidden">
        <Show when={resourcePacks.length > 0} fallback={<NoResourcePacks />}>
          <For each={resourcePacks}>
            {(props) => <ResourcePack resourcePack={props} />}
          </For>
        </Show>
      </div>
    </div>
  )
}

export default ResourcePacks
