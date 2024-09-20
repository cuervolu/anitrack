import {ref} from 'vue'
import {useToast} from "@/components/ui/toast"
import {check} from '@tauri-apps/plugin-updater'
import {relaunch} from '@tauri-apps/plugin-process'
import {info, error as logError} from "@tauri-apps/plugin-log";

export function useUpdater() {
  const {toast} = useToast()
  const isCheckingForUpdate = ref(false)
  const isUpdating = ref(false)
  const updateAvailable = ref(false)
  const downloadProgress = ref(0)

  async function checkForUpdates() {
    isCheckingForUpdate.value = true
    try {
      const update = await check()
      if (update) {
        updateAvailable.value = true
        toast({
          title: "Actualización disponible",
          description: `La versión ${update.version} está disponible para descargar.`,
        })
        return update
      } else {
        toast({
          title: "No hay actualizaciones",
          description: "Estás utilizando la versión más reciente de la aplicación.",
        })
      }
    } catch (error) {
      await logError(`Error al buscar actualizaciones: ${error}`)
      toast({
        title: "Error",
        description: "No se pudo buscar actualizaciones.",
        variant: "destructive"
      })
    } finally {
      isCheckingForUpdate.value = false
    }
    return null
  }

  async function downloadAndInstallUpdate() {
    isUpdating.value = true
    try {
      const update = await checkForUpdates()
      if (update) {
        let downloaded = 0
        let contentLength = 0
        await update.downloadAndInstall((event) => {
          switch (event.event) {
            case 'Started':
              contentLength = event.data.contentLength || 0
              info(`Comenzó la descarga de ${contentLength} bytes`)
              break
            case 'Progress':
              downloaded += event.data.chunkLength
              downloadProgress.value = (downloaded / contentLength) * 100
              info(`Descargado ${downloaded} de ${contentLength}`)
              break
            case 'Finished':
              info('Actualización descargada')
              break
          }
        })
        await info('Actualización instalada')
        await relaunch()
      }
    } catch (error) {
      await logError(`Error al descargar o instalar la actualización: ${error}`)
      toast({
        title: "Error",
        description: "No se pudo descargar o instalar la actualización.",
        variant: "destructive"
      })
    } finally {
      isUpdating.value = false
    }
  }

  return {
    isCheckingForUpdate,
    isUpdating,
    updateAvailable,
    downloadProgress,
    checkForUpdates,
    downloadAndInstallUpdate
  }
}