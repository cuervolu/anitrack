<script lang="ts" setup>
import {Search, PlusCircle} from 'lucide-vue-next'
import {Button} from '@/components/ui/button'
import {Input} from '@/components/ui/input'
import {ScrollArea} from '@/components/ui/scroll-area'
</script>

<template>
  <div>
    <header class="p-4 border-b flex justify-between items-center">
      <div class="flex items-center">
        <Input placeholder="Buscar anime..." class="w-64 mr-2"/>
        <Button variant="ghost" size="icon">
          <Search class="h-4 w-4"/>
        </Button>
      </div>
      <Button>
        <PlusCircle class="mr-2 h-4 w-4"/>
        Añadir Anime
      </Button>
    </header>
    <ScrollArea class="flex-1 p-4">
      <h2 class="text-2xl font-bold mb-4">Lista de Animes</h2>
      <div class="grid gap-4">
        <div v-for="i in 5" :key="i"
             class="flex items-center space-x-4 p-4 bg-muted/40 rounded-lg shadow">
          <img
              :src="`/placeholder.svg?height=80&width=60&text=Anime ${i}`"
              :alt="`Anime ${i}`"
              class="w-15 h-20 object-cover rounded"
          />
          <div>
            <h3 class="font-bold">Anime Título {{ i }}</h3>
            <p class="text-sm text-gray-500">Episodios vistos: 5/12</p>
          </div>
        </div>
      </div>
    </ScrollArea>
  </div>
</template>

