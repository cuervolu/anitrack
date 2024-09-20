<script lang="ts" setup>
import {ref, onMounted} from 'vue'
import {Search, PlusCircle} from 'lucide-vue-next'
import {Button} from '@/components/ui/button'
import {Input} from '@/components/ui/input'
import {ScrollArea} from '@/components/ui/scroll-area'
import AnimeCard from '@/components/AnimeCard.vue'
import {useAnimeStore} from '~/stores'

const animeStore = useAnimeStore()
const searchQuery = ref('')

onMounted(async () => {
  await animeStore.fetchAnimes()
})

const filteredAnimes = computed(() => {
  return animeStore.animes.filter(anime =>
      anime.title.toLowerCase().includes(searchQuery.value.toLowerCase())
  )
})
</script>

<template>
  <div>
    <header class="p-4 border-b flex justify-between items-center">
      <div class="flex items-center">
        <Input v-model="searchQuery" placeholder="Buscar anime..." class="w-64 mr-2"/>
        <Button variant="ghost" size="icon">
          <Search class="h-4 w-4"/>
        </Button>
      </div>
      <Button as-child>
        <NuxtLink to="/create">
          <PlusCircle class="mr-2 h-4 w-4"/>
          Añadir Animes
        </NuxtLink>
      </Button>
    </header>
    <ScrollArea class="flex-1 p-4">
      <h2 class="text-2xl font-bold mb-4">Lista de Animes</h2>
      <div v-if="animeStore.loading" class="text-center">
        Cargando animes...
      </div>
      <div v-else-if="animeStore.error" class="text-center text-destructive">
        {{ animeStore.error }}
      </div>
      <div v-else-if="filteredAnimes.length === 0" class="text-center">
        No se encontraron animes.
      </div>
      <div v-else class="grid gap-4">
        <AnimeCard
            v-for="anime in filteredAnimes"
            :key="anime.id"
            :anime="anime"
        />
      </div>
    </ScrollArea>
  </div>
</template>