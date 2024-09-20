<script setup lang="ts">
import {Card, CardContent, CardDescription, CardHeader, CardTitle} from "@/components/ui/card";
import {Tv, ListPlus, BarChart2, Search, Heart, Clock} from "lucide-vue-next";

const features = [
  {
    icon: ListPlus,
    title: "Anime Management",
    description: "Add, edit, and remove anime from your collection with ease."
  },
  {
    icon: Tv,
    title: "Viewing and Tracking",
    description: "Keep track of your watching progress and view detailed information about each anime."
  },
  {
    icon: BarChart2,
    title: "Categorization",
    description: "Sort and categorize your anime by status, completion, and genre."
  },
  {
    icon: Heart,
    title: "Customizable Lists",
    description: "Create and manage custom lists, including your favorites."
  },
  {
    icon: Search,
    title: "Search and Filtering",
    description: "Easily find anime in your collection with powerful search and filtering options."
  },
  {
    icon: Clock,
    title: "Statistics",
    description: "View your watching statistics and total viewing time."
  },
];
</script>

<template>
  <div class="space-y-6">
    <h2 class="text-3xl font-bold tracking-tight">Welcome to AniTrack</h2>
    <p class="text-muted-foreground">
      Manage and track your favorite anime series and movies with ease.
    </p>
    <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
      <Card v-for="(feature, index) in features" :key="index">
        <CardHeader>
          <div class="flex items-center gap-4">
            <component :is="feature.icon" class="h-8 w-8"/>
            <CardTitle>{{ feature.title }}</CardTitle>
          </div>
        </CardHeader>
        <CardContent>
          <CardDescription>{{ feature.description }}</CardDescription>
        </CardContent>
      </Card>
    </div>
  </div>
</template>

