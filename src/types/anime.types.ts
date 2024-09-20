export interface Anime {
  id: number
  title: string
  description: string | null
  image_path: string | null
  total_episodes: number | null
  release_date: string | null
  end_date: string | null
  status: 'En emisión' | 'Terminado'
  user_status: 'Visto' | 'No visto' | 'En progreso' | 'Planificado'
  user_rating: number | null
}

export interface Genre {
  id: number
  name: string
}

export interface Episode {
  id: number
  anime_id: number
  episode_number: number
  title: string | null
  duration: number | null
  is_watched: boolean
  is_favorite: boolean
}

export type AnimeWithoutId = Omit<Anime, 'id'>