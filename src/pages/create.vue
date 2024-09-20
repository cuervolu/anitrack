<script setup lang="ts">

import {ArrowLeft} from 'lucide-vue-next'
import AnimeForm from "~/components/AnimeForm.vue";


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
    <anime-form/>
  </div>
</template>

<style scoped>

</style>