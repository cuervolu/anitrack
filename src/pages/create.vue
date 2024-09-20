<script setup lang="ts">

import {ArrowLeft} from 'lucide-vue-next'
import AnimeForm from "~/components/AnimeForm.vue";
import type {AnimeWithoutId} from "~/types";


import {useRouter} from "#vue-router";
const animeStore = useAnimeStore()
const router = useRouter()

const handleSubmit = async (anime: AnimeWithoutId) => {
  await animeStore.addAnime(anime)
  await router.push('/')
}

</script>

<template>
  <div class="container mx-auto px-4 py-5">
    <div class="mb-6">
      <Button variant="ghost" asChild>
        <NuxtLink href="/">
          <ArrowLeft class="mr-2 h-4 w-4"/>
          Volver al inicio
        </NuxtLink>
      </Button>
    </div>
    <anime-form  @submit="handleSubmit"/>
  </div>
</template>

<style scoped>

</style>