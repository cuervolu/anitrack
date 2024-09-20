<script lang="ts" setup>
import { Card, CardContent } from '@/components/ui/card'
import type { Anime } from '@/types'

defineProps<{
  anime: Anime
}>()
</script>

<template>
  <NuxtLink :to="`/anime/${anime.id}`" class="block">
    <Card class="transition-shadow hover:shadow-md">
      <CardContent class="flex items-center space-x-4 p-4">
        <img
            :src="anime.image_path || `/placeholder.svg?height=80&width=60&text=${anime.title}`"
            :alt="anime.title"
            class="w-15 h-20 object-cover rounded"
        />
        <div>
          <h3 class="font-bold">{{ anime.title }}</h3>
          <p class="text-sm text-muted-foreground">
            Estado: {{ anime.user_status }}
          </p>
          <p v-if="anime.total_episodes" class="text-sm text-muted-foreground">
            Episodios: {{ anime.total_episodes }}
          </p>
        </div>
      </CardContent>
    </Card>
  </NuxtLink>
</template>