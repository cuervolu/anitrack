<script setup lang="ts">
import {computed} from 'vue'
import {RadioGroup, RadioGroupItem} from '@/components/ui/radio-group'
import {Label} from '@/components/ui/label'
import {Moon, Sun, Laptop} from 'lucide-vue-next'

const colorMode = useColorMode()

const currentMode = computed({
  get: () => colorMode.preference,
  set: (value) => {
    colorMode.preference = value
  }
})
</script>

<template>
  <RadioGroup v-model="currentMode" class="flex space-x-2">
    <div class="flex items-center space-x-2">
      <RadioGroupItem value="light" id="light"/>
      <Label htmlFor="light" class="flex items-center cursor-pointer">
        <Sun class="h-4 w-4 mr-2"/>
        Claro
      </Label>
    </div>
    <div class="flex items-center space-x-2">
      <RadioGroupItem value="dark" id="dark"/>
      <Label htmlFor="dark" class="flex items-center cursor-pointer">
        <Moon class="h-4 w-4 mr-2"/>
        Oscuro
      </Label>
    </div>
    <div class="flex items-center space-x-2">
      <RadioGroupItem value="system" id="system"/>
      <Label htmlFor="system" class="flex items-center cursor-pointer">
        <Laptop class="h-4 w-4 mr-2"/>
        Sistema
      </Label>
    </div>
  </RadioGroup>
</template>