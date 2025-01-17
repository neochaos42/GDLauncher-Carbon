import { rspc } from "@/utils/rspcClient"
import { Mod } from "@gd/core_module/bindings"
import { createMemo } from "solid-js"
import { createStore, reconcile } from "solid-js/store"

//@ts-ignore
const fetchData = ({ params }) => {
  const instanceDetails = rspc.createQuery(() => ({
    queryKey: ["instance.getInstanceDetails", parseInt(params.id, 10)]
  }))

  const modpackInfo = rspc.createQuery(() => ({
    queryKey: ["instance.getModpackInfo", parseInt(params.id, 10)]
  }))

  const instancesUngrouped = rspc.createQuery(() => ({
    queryKey: ["instance.getAllInstances"]
  }))

  const _instanceMods = rspc.createQuery(() => ({
    queryKey: ["instance.getInstanceMods", parseInt(params.id, 10)]
  }))

  const [instanceMods, setInstanceMods] = createStore({
    mods: [] as Mod[]
  })

  createMemo(() => {
    const mods = _instanceMods.data
    setInstanceMods("mods", reconcile(mods || []))
  })

  const totalRam = rspc.createQuery(() => ({
    queryKey: ["systeminfo.getTotalRAM"]
  }))

  return {
    instanceDetails,
    modpackInfo,
    instanceMods: instanceMods.mods,
    instancesUngrouped,
    totalRam
  }
}

export default fetchData
