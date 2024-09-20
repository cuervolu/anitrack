<script setup lang="ts">
import { ref, computed } from 'vue'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { useConfigStore } from '~/stores'

const configStore = useConfigStore()

const fonts = [
  {value: 'Montserrat', label: 'Montserrat'},
  {value: 'Roboto', label: 'Roboto'},
  {value: 'Open Sans', label: 'Open Sans'},
  {value: "Karla", label: "Karla"},
  {value: "Inter", label: "Inter"},
  {value: "Geist Mono", label: "Geist Mono"},
]

const selectedFont = computed(() => configStore.config.value.font)

const handleFontChange = async (newFont: string) => {
  await configStore.setFont(newFont)
}
</script>

<template>
  <div>
    <h2 class="text-xl font-semibold mb-2">Fuente de texto</h2>
    <Select :model-value="selectedFont" @update:model-value="handleFontChange">
      <SelectTrigger class="w-[180px]">
        <SelectValue placeholder="Selecciona una fuente" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem v-for="font in fonts" :key="font.value" :value="font.value">
          {{ font.label }}
        </SelectItem>
      </SelectContent>
    </Select>
  </div>
</template>