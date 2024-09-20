<script lang="ts" setup>
import {Tv, Home, Calendar, Settings} from 'lucide-vue-next'
import {Button} from '@/components/ui/button'
import {Separator} from '@/components/ui/separator'
</script>

<template>
  <div class="w-64 bg-muted/60 text-foreground h-full flex flex-col">
    <div class="p-4 border-b border-slate-700">
      <h1 class="text-2xl font-bold flex items-center">
        <img src="~/assets/img/logo.svg" alt="Logo" class="mr-2 w-10 h-10">
<!--        <Tv class="mr-2"/>-->
        AnimeTracker
      </h1>
    </div>
    <nav class="flex-1 py-4">
      <Button variant="ghost" class="w-full justify-start py-2">
        <NuxtLink to="/" class="flex items-center w-full">
          <Home class="mr-3 h-5 w-5"/>
          Inicio
        </NuxtLink>
      </Button>
      <Button variant="ghost" class="w-full justify-start py-2">
        <NuxtLink to="/calendario" class="flex items-center w-full">
          <Calendar class="mr-3 h-5 w-5"/>
          Calendario
        </NuxtLink>
      </Button>
      <Button variant="ghost" class="w-full justify-start py-2">
        <NuxtLink to="/configuracion" class="flex items-center w-full">
          <Settings class="mr-3 h-5 w-5"/>
          Configuración
        </NuxtLink>
      </Button>
    </nav>
    <div class="mt-auto p-4 mb-5">
      <Separator class="my-2"/>
      <p class="text-sm text-center ">Made with ❤️ by Cuervolu</p>
    </div>
  </div>
</template>