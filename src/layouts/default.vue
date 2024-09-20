<!-- layouts/default.vue -->
<script setup lang="ts">
import { Search } from 'lucide-vue-next'
import { Input } from "@/components/ui/input"
import Sidebar from "~/components/Sidebar.vue";

useColorMode();
</script>

<template>
  <div class="flex h-screen bg-background">
    <Sidebar />
    <div class="flex-1 flex flex-col">
      <header class="flex h-14 items-center gap-4 border-b bg-muted/40 px-6">
        <div class="flex-1">
          <h1 class="text-lg font-semibold">AniTrack</h1>
        </div>
        <div class="flex items-center gap-4">
          <form>
            <div class="relative">
              <Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
              <Input
                  type="search"
                  placeholder="Search anime..."
                  class="w-64 pl-8"
              />
            </div>
          </form>
        </div>
      </header>
      <main class="flex-1 overflow-y-auto">
        <div class="container mx-auto p-6">
          <slot></slot>
        </div>
      </main>
    </div>
  </div>
</template>