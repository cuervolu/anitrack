<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { ArrowLeft, Calendar, Star, Play, Edit } from 'lucide-vue-next'
import { useAnimeStore } from '@/stores'
import { Badge } from "~/components/ui/badge"
import { Button } from "~/components/ui/button"
import { Progress } from "~/components/ui/progress"
import { Separator } from "~/components/ui/separator"
import { Skeleton } from "~/components/ui/skeleton"
import type { Anime, Episode } from '@/types'

const route = useRoute()
const animeStore = useAnimeStore()
const id = Number(route.params.id)
const animeData = ref<Anime | null>(null)
const episodes = ref<Episode[]>([])

onMounted(async () => {
  animeData.value = await animeStore.getAnimeById(id)
  episodes.value = await animeStore.getEpisodesByAnimeId(id)
})

const watchedEpisodes = computed(() => {
  return episodes.value.filter(episode => episode.is_watched).length
})

const watchedPercentage = computed(() => {
  if (!animeData.value?.total_episodes) return 0
  return (watchedEpisodes.value / animeData.value.total_episodes) * 100
})
</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <div class="mb-6">
      <NuxtLink to="/" class="inline-flex items-center">
        <Button variant="ghost">
          <ArrowLeft class="mr-2 h-4 w-4"/>
          Volver al listado
        </Button>
      </NuxtLink>
    </div>
    <div v-if="animeStore.loading" class="grid md:grid-cols-3 gap-8">
      <div class="md:col-span-1">
        <Skeleton class="w-full h-[600px] rounded-lg" />
      </div>
      <div class="md:col-span-2 space-y-6">
        <Skeleton class="h-12 w-3/4" />
        <Skeleton class="h-20 w-full" />
        <div class="flex flex-wrap gap-2">
          <Skeleton class="h-6 w-24" />
          <Skeleton class="h-6 w-24" />
        </div>
        <div class="flex items-center space-x-2">
          <Skeleton class="h-6 w-6" />
          <Skeleton class="h-6 w-16" />
        </div>
        <div class="space-y-2">
          <Skeleton class="h-6 w-48" />
          <Skeleton class="h-6 w-48" />
        </div>
        <Separator />
        <div class="space-y-2">
          <Skeleton class="h-8 w-32" />
          <Skeleton class="h-6 w-full" />
          <Skeleton class="h-4 w-full" />
        </div>
        <div class="flex space-x-4">
          <Skeleton class="h-10 w-40" />
          <Skeleton class="h-10 w-40" />
        </div>
      </div>
    </div>
    <div v-else-if="animeData" class="grid md:grid-cols-3 gap-8">
      <div class="md:col-span-1">
        <img
            :src="animeData.image_path || '/placeholder.svg?height=600&width=400&text=No Image'"
            :alt="animeData.title"
            class="rounded-lg shadow-lg w-full"
        />
      </div>
      <div class="md:col-span-2 space-y-6">
        <h1 class="text-4xl font-bold">{{ animeData.title }}</h1>
        <p class="text-lg text-muted-foreground">{{ animeData.description }}</p>
        <div class="flex flex-wrap gap-2">
          <Badge variant="secondary">{{ animeData.status }}</Badge>
          <Badge variant="outline">{{ animeData.user_status }}</Badge>
        </div>
        <div class="flex items-center space-x-2">
          <Star class="text-yellow-400"/>
          <span class="text-xl font-semibold">{{
              animeData.user_rating?.toFixed(1) || 'N/A'
            }}</span>
        </div>
        <div class="space-y-2">
          <div class="flex items-center space-x-2">
            <Calendar class="text-gray-500"/>
            <span>Estreno: {{ animeData.release_date }}</span>
          </div>
          <div v-if="animeData.end_date" class="flex items-center space-x-2">
            <Calendar class="text-gray-500"/>
            <span>Finalización: {{ animeData.end_date }}</span>
          </div>
        </div>
        <Separator/>
        <div class="space-y-2">
          <h2 class="text-2xl font-semibold">Progreso</h2>
          <div class="flex items-center justify-between">
            <span>Episodios vistos: {{ watchedEpisodes }}/{{ animeData.total_episodes }}</span>
            <span class="text-sm text-gray-500">
              {{ watchedPercentage.toFixed(0) }}%
            </span>
          </div>
          <Progress :value="watchedPercentage" class="w-full"/>
        </div>
        <div class="flex space-x-4">
          <Button>
            <Play class="mr-2 h-4 w-4"/>
            Continuar viendo
          </Button>
          <Button variant="outline" as-child>
            <NuxtLink :to="`/anime/edit/${animeData.id}`">
              <Edit class="mr-2 h-4 w-4"/>
              Editar información
            </NuxtLink>
          </Button>
        </div>
      </div>
    </div>
    <div v-else-if="animeStore.error" class="text-center text-destructive">
      {{ animeStore.error }}
    </div>
    <div v-else class="text-center">
      No se encontró el anime
    </div>
  </div>
</template>