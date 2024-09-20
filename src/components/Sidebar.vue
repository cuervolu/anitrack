<!-- components/Sidebar.vue -->
<script setup lang="ts">
import {
  ListPlus,
  Edit,
  Trash2,
  List,
  Eye,
  BarChart2,
  Heart,
  Clock,
  type LucideIcon
} from 'lucide-vue-next'
import {Button} from "@/components/ui/button"
import {ScrollArea} from "@/components/ui/scroll-area"

type ButtonVariant =
    "default"
    | "destructive"
    | "outline"
    | "secondary"
    | "ghost"
    | "link"
    | null
    | undefined

interface MenuItem {
  label: string;
  icon: LucideIcon;
  variant: ButtonVariant;
}

interface MenuSection {
  title: string;
  items: MenuItem[];
}

const menuItems: MenuSection[] = [
  {
    title: 'Manage Anime',
    items: [
      {label: 'Add New Anime', icon: ListPlus, variant: 'secondary'},
      {label: 'Edit Anime', icon: Edit, variant: 'ghost'},
      {label: 'Remove Anime', icon: Trash2, variant: 'ghost'},
    ],
  },
  {
    title: 'View Anime',
    items: [
      {label: 'All Anime', icon: List, variant: 'ghost'},
      {label: 'Anime Details', icon: Eye, variant: 'ghost'},
    ],
  },
  {
    title: 'Categories',
    items: [
      {label: 'By Status', icon: BarChart2, variant: 'ghost'},
      {label: 'By Completion', icon: Clock, variant: 'ghost'},
    ],
  },
  {
    title: 'Lists',
    items: [
      {label: 'Favorites', icon: Heart, variant: 'ghost'},
    ],
  },
]
</script>

<template>
  <div class="w-64 h-full border-r bg-background">
    <ScrollArea class="h-full">
      <div class="space-y-4 py-4">
        <div v-for="(section, index) in menuItems" :key="index" class="px-3 py-2">
          <h2 class="mb-2 px-4 text-lg font-semibold tracking-tight">
            {{ section.title }}
          </h2>
          <div class="space-y-1">
            <Button
                v-for="(item, itemIndex) in section.items"
                :key="itemIndex"
                :variant="item.variant"
                class="w-full justify-start"
            >
              <component :is="item.icon" class="mr-2 h-4 w-4"/>
              {{ item.label }}
            </Button>
          </div>
        </div>
      </div>
    </ScrollArea>
  </div>
</template>