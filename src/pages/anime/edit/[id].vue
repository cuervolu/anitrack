<script setup lang="ts">
import {ref, onMounted} from 'vue'
import {useRoute, useRouter} from 'vue-router'
import AnimeForm from '@/components/AnimeForm.vue'
import {useAnimeStore} from '~/stores'
import {Skeleton} from '@/components/ui/skeleton'
import type {Anime, AnimeWithoutId} from '@/types'

const route = useRoute()
const router = useRouter()
const animeStore = useAnimeStore()
const anime = ref<Anime | null>(null)
const loading = ref(true)

onMounted(async () => {
  const id = parseInt(route.params.id as string)
  if (!isNaN(id)) {
    anime.value = await animeStore.getAnimeById(id)
  }
  loading.value = false
})

const handleSubmit = async (updatedAnime: AnimeWithoutId) => {
  if (anime.value) {
    await animeStore.updateAnime(anime.value.id, updatedAnime)
    await router.push(`/anime/${anime.value.id}`)
  }
}
</script>

<template>
  <div v-if="loading" class="space-y-6 max-w-2xl mx-auto p-6 bg-muted/40 rounded-lg shadow">
    <Skeleton class="h-8 w-3/4 mx-auto mb-6"/>
    <Skeleton class="h-10 w-full"/>
    <Skeleton class="h-32 w-full"/>
    <Skeleton class="h-10 w-full"/>
    <Skeleton class="h-10 w-full"/>
    <div class="grid grid-cols-2 gap-4">
      <Skeleton class="h-10 w-full"/>
      <Skeleton class="h-10 w-full"/>
    </div>
    <Skeleton class="h-10 w-full"/>
    <Skeleton class="h-10 w-full"/>
    <Skeleton class="h-10 w-full"/>
    <Skeleton class="h-10 w-full"/>
  </div>
  <div v-else-if="anime">
    <AnimeForm :anime="anime" @submit="handleSubmit"/>
  </div>
  <div v-else class="text-center text-destructive">
    No se pudo cargar el anime.
  </div>
</template>