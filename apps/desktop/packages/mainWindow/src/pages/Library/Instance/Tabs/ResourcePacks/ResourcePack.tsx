import { Checkbox, Switch } from "@gd/ui"

interface IResourcePack {
  title: string
  enabled: boolean
  mcversion: string
  modloaderVersion: string
  resourcePackVersion: string
}

interface Props {
  resourcePack: IResourcePack
}

const ResourcePack = (props: Props) => {
  return (
    <div class="w-full h-14 flex items-center py-2 box-border">
      <div class="flex gap-4 justify-between items-center w-full">
        <div class="flex gap-4 justify-between items-center">
          <Checkbox checked={true} disabled={false} />
          <div class="flex items-center gap-2">
            <div class="h-10 w-10 rounded-xl bg-green-500" />
            <div class="flex flex-col">
              {props.resourcePack.title}
              <div class="flex gap-2">
                <p class="m-0 text-lightSlate-700 text-sm">
                  {props.resourcePack.resourcePackVersion}
                </p>
              </div>
            </div>
          </div>
        </div>
        <Switch />
      </div>
    </div>
  )
}

export default ResourcePack
