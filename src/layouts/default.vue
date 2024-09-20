<script setup lang="ts">
import Sidebar from "~/components/Sidebar.vue";
import TitleBar from "~/components/TitleBar.vue";

useColorMode();
</script>

<template>
  <div class="flex flex-col h-screen">
    <TitleBar title="main" class="fixed top-0 left-0 right-0 z-50" bg-color="bg-muted/60"/>
    <div class="flex flex-1 mt-5">
      <Sidebar class="mt-5"/>
      <div class="flex-1 flex flex-col overflow-auto mt-10 relative">
        <div class="absolute inset-0 overflow-auto">
          <slot/>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>

</style>