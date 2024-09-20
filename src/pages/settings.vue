<script setup lang="ts">
import {getVersion, getName} from "@tauri-apps/api/app"
import {appLogDir, appConfigDir} from '@tauri-apps/api/path'
import {open as openDirectory} from '@tauri-apps/plugin-shell'
import {error as logError} from "@tauri-apps/plugin-log";

import {ArrowLeft, RefreshCw, Folder, Download} from "lucide-vue-next"

import {Button} from "@/components/ui/button"
import {Label} from "@/components/ui/label"
import {Separator} from "@/components/ui/separator"
import {useToast} from "@/components/ui/toast"
import ModeToggle from "~/components/ModeToggle.vue"
import {useUpdater} from '~/composables/useUpdater'
import FontSelector from "~/components/FontSelector.vue";

const {toast} = useToast()
const appVersion = await getVersion()
const appName = await getName()
const logDir = await appLogDir()
const configDir = await appConfigDir()

const {
  isCheckingForUpdate,
  isUpdating,
  updateAvailable,
  downloadProgress,
  checkForUpdates,
  downloadAndInstallUpdate
} = useUpdater()

const openFolder = async (path: string) => {
  try {
    await openDirectory(path)
  } catch (error) {
    await logError(`Error al abrir la carpeta: ${error}`)
    toast({
      title: "Error",
      description: "No se pudo abrir la carpeta.",
      variant: "destructive"
    })
  }
}
</script>

<template>
  <div class="container mx-auto px-4 py-2 pb-5">
    <div class="mb-6">
      <Button variant="ghost" as-child>
        <NuxtLink to="/">
          <ArrowLeft class="mr-2 h-4 w-4"/>
          Volver al inicio
        </NuxtLink>
      </Button>
    </div>
    <h1 class="text-3xl font-bold mb-6">Configuración</h1>
    <div class="space-y-6">
      <div>
        <h2 class="text-xl font-semibold mb-2">Tema</h2>
        <ModeToggle/>
      </div>
      <Separator/>
      <FontSelector />
      <Separator/>
      <div>
        <h2 class="text-xl font-semibold mb-2">Ubicaciones de archivos</h2>
        <div class="space-y-2">
          <div>
            <Label class="text-sm font-medium">Logs de la aplicación:</Label>
            <p class="text-sm text-muted-foreground">{{ logDir }}</p>
            <Button @click="() => openFolder(logDir)" class="mt-2" variant="outline" size="sm">
              <Folder class="mr-2 h-4 w-4"/>
              Abrir carpeta de logs
            </Button>
          </div>
          <div>
            <Label class="text-sm font-medium">Configuración de la aplicación:</Label>
            <p class="text-sm text-muted-foreground">{{ configDir }}</p>
            <Button @click="() => openFolder(configDir)" class="mt-2" variant="outline" size="sm">
              <Folder class="mr-2 h-4 w-4"/>
              Abrir carpeta de configuración
            </Button>
          </div>
        </div>
      </div>
      <Separator/>
      <div>
        <h2 class="text-xl font-semibold mb-2">Información de la aplicación</h2>
        <p class="text-sm text-muted-foreground">Versión: {{ appVersion }}</p>
        <p class="text-sm text-muted-foreground">Nombre: {{ appName }}</p>
        <Button @click="checkForUpdates" class="mt-2" :disabled="isCheckingForUpdate || isUpdating">
          <RefreshCw class="mr-2 h-4 w-4"/>
          {{ isCheckingForUpdate ? 'Buscando actualizaciones...' : 'Buscar actualizaciones' }}
        </Button>
        <Button v-if="updateAvailable" @click="downloadAndInstallUpdate" class="mt-2 ml-2"
                :disabled="isUpdating">
          <Download class="mr-2 h-4 w-4"/>
          {{
            isUpdating ? `Descargando (${downloadProgress.toFixed(0)}%)` : 'Descargar e instalar actualización'
          }}
        </Button>
      </div>
    </div>
  </div>
</template>