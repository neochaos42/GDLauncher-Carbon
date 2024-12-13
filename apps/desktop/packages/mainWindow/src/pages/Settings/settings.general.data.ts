import { rspc } from "@/utils/rspcClient"

const SettingsJavaData = () => {
  const data = rspc.createQuery(() => ({
    queryKey: ["settings.getSettings"]
  }))
  return { data }
}

export default SettingsJavaData
