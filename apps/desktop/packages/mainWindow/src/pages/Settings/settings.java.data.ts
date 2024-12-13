import { rspc } from "@/utils/rspcClient"

const SettingsJavaData = () => {
  const availableJavas = rspc.createQuery(() => ({
    queryKey: ["java.getAvailableJavas"]
  }))
  const javaProfiles = rspc.createQuery(() => ({
    queryKey: ["java.getJavaProfiles"]
  }))
  const totalRam = rspc.createQuery(() => ({
    queryKey: ["systeminfo.getTotalRAM"]
  }))
  return { availableJavas, javaProfiles, totalRam }
}

export default SettingsJavaData
