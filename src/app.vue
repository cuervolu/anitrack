<script setup lang="ts">
import {listen} from '@tauri-apps/api/event';
import {check} from '@tauri-apps/plugin-updater';
import {relaunch} from '@tauri-apps/plugin-process';
import {ask} from '@tauri-apps/plugin-dialog';

import {useToast} from "@/components/ui/toast";

import Toaster from '@/components/ui/toast/Toaster.vue'
import {error as logError} from "@tauri-apps/plugin-log";

const {toast} = useToast();

onMounted(async () => {
  await listen('update-available', async (event: any) => {
    const version = event.payload;
    const shouldUpdate = await ask(`Hay una nueva versión disponible (${version}). ¿Deseas actualizar ahora?`, {
      title: 'Actualización disponible',
      kind: 'info',
    });

    if (shouldUpdate) {
      try {
        const update = await check();
        if (update) {
          await update.downloadAndInstall();
          await relaunch();
        }
      } catch (error) {
        await logError(`Error al actualizar: ${error}`);
        toast({
          title: "Error",
          description: "No se pudo completar la actualización.",
          variant: "destructive"
        });
      }
    }
  });
});
</script>


<template>
  <NuxtLayout>
    <Toaster/>
    <NuxtPage/>
  </NuxtLayout>
</template>
