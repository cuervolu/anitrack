import {defineStore} from 'pinia'
import type {Anime, AnimeWithoutId, Episode} from '@/types/'
import {invoke} from '@tauri-apps/api/core'
import {info, error as logError} from "@tauri-apps/plugin-log";

export const useAnimeStore = defineStore('anime', () => {
  const animes = ref<Anime[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const fetchAnimes = async () => {
    loading.value = true
    try {
      animes.value = await invoke<Anime[]>('fetch_animes')
      loading.value = false
    } catch (err) {
      error.value = 'No se pudieron cargar los animes. Por favor, inténtalo de nuevo más tarde.'
      loading.value = false
    }
  }

  const addAnime = async (anime: AnimeWithoutId) => {
    loading.value = true
    try {
      await info("Adding anime: " + JSON.stringify(anime));
      const newAnime = await invoke<Anime>('add_anime', {anime})
      animes.value.push(newAnime)
      loading.value = false
    } catch (err) {
      await logError(`Error: ${err}`);
      error.value = 'No se pudo agregar el anime. Por favor, verifica los datos e inténtalo de nuevo.'
      loading.value = false
    }
  }

  const updateAnime = async (id: number, updates: Partial<Anime>) => {
    loading.value = true
    try {
      const updatedAnime = await invoke<Anime>('update_anime', {id, anime: updates})
      const index = animes.value.findIndex(anime => anime.id === id)
      if (index !== -1) {
        animes.value[index] = updatedAnime
      }
      loading.value = false
    } catch (err) {
      await logError(`Error: ${err}`);
      error.value = 'No se pudo actualizar el anime. Por favor, verifica los cambios e inténtalo de nuevo.'
      loading.value = false
    }
  }

  const deleteAnime = async (id: number) => {
    loading.value = true
    try {
      await invoke<void>('delete_anime', {id})
      animes.value = animes.value.filter(anime => anime.id !== id)
      loading.value = false
    } catch (err) {
      error.value = 'No se pudo eliminar el anime. Por favor, inténtalo de nuevo más tarde.'
      loading.value = false
    }
  }
  
  const getAnimeById = async (id: number): Promise<Anime | null> => {
    loading.value = true
    try {
      const anime = await invoke<Anime>('get_anime_by_id', {id})
      loading.value = false
      return anime
    } catch (err) {
      error.value = 'No se pudo obtener el anime. Por favor, inténtalo de nuevo más tarde.'
      loading.value = false
      return null
    }
  }

  const getEpisodesByAnimeId = async (animeId: number): Promise<Episode[]> => {
    loading.value = true
    try {
      const episodes = await invoke<Episode[]>('get_episodes_by_anime_id', { animeId })
      loading.value = false
      return episodes
    } catch (err) {
      error.value = 'No se pudieron obtener los episodios. Por favor, inténtalo de nuevo más tarde.'
      loading.value = false
      return []
    }
  }

  return {
    animes,
    loading,
    error,
    fetchAnimes,
    addAnime,
    updateAnime,
    deleteAnime,
    getAnimeById,
    getEpisodesByAnimeId,
  }
})