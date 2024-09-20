<script setup lang="ts">
import {ref, onMounted} from 'vue'
import {useRoute, useRouter} from 'vue-router'
import AnimeForm from '@/components/AnimeForm.vue'
import {useAnimeStore} from '~/stores'
import type {Anime, AnimeWithoutId} from '@/types'

const route = useRoute()
const router = useRouter()
const animeStore = useAnimeStore()

const anime = ref<Anime | null>(null)

onMounted(async () => {
  const id = parseInt(route.params.id as string)
  if (!isNaN(id)) {
    anime.value = await animeStore.getAnimeById(id)
  }
})

const handleSubmit = async (updatedAnime: AnimeWithoutId) => {
  if (anime.value) {
    await animeStore.updateAnime(anime.value.id, updatedAnime)
    await router.push(`/anime/${anime.value.id}`)
  }
}
</script>

<template>
  <div v-if="anime">
    <AnimeForm :anime="anime" @submit="handleSubmit"/>
  </div>
  <div v-else>
    <p>Cargando anime...</p>
  </div>
</template>