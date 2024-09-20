<script setup lang="ts">
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';

import { CalendarDate, type DateValue } from '@internationalized/date';
import { CalendarIcon, UploadIcon } from 'lucide-vue-next'
import { useForm } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/zod'

import * as z from 'zod'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Calendar } from '@/components/ui/calendar'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import { FormField, FormItem, FormLabel, FormControl, FormMessage } from '@/components/ui/form'
import { useToast } from "@/components/ui/toast"

import { useAnimeStore } from '~/stores'
import type { AnimeWithoutId, Anime } from '@/types'

const props = defineProps<{
  anime?: Anime
}>()

const emit = defineEmits<{
  (e: 'submit', anime: AnimeWithoutId): void
}>()

const animeStore = useAnimeStore()
const { toast } = useToast()

const formSchema = toTypedSchema(z.object({
  title: z.string().min(1, 'El título es requerido'),
  description: z.string().nullable(),
  image_path: z.string().nullable(),
  total_episodes: z.number().nullable(),
  release_date: z.string().nullable(),
  end_date: z.string().nullable(),
  status: z.enum(['En emisión', 'Terminado']),
  user_status: z.enum(['Visto', 'No visto', 'En progreso', 'Planificado']),
  user_rating: z.number().min(0).max(10).nullable()
}))

const form = useForm({
  validationSchema: formSchema,
  initialValues: props.anime || {
    title: "",
    description: null,
    image_path: null,
    total_episodes: null,
    release_date: null,
    end_date: null,
    status: 'En emisión',
    user_status: 'No visto',
    user_rating: null
  }
})

const parseDate = (dateString: string | null): DateValue | undefined => {
  if (!dateString) return undefined;
  const [year, month, day] = dateString.split('-').map(Number);
  return new CalendarDate(year, month, day);
};

const handleImageUpload = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Image',
        extensions: ['png', 'jpg', 'jpeg', 'gif']
      }]
    });

    if (selected) {
      const savedPath = await invoke('save_image', { sourcePath: selected as string });
      const assetUrl = convertFileSrc(savedPath as string);
      form.setFieldValue('image_path', assetUrl);
    }
  } catch (err) {
    console.error('Failed to save image', err);
    toast({
      title: "Error",
      description: "No se pudo guardar la imagen.",
      variant: "destructive",
    });
  }
}

const handleDateChange = (name: 'release_date' | 'end_date', date: DateValue | undefined) => {
  if (date instanceof CalendarDate) {
    form.setFieldValue(name, `${date.year}-${date.month.toString().padStart(2, '0')}-${date.day.toString().padStart(2, '0')}`);
  } else {
    form.setFieldValue(name, null);
  }
};

const formatDate = (dateString: string | null) => {
  if (!dateString) return 'No date selected';
  const date = parseDate(dateString);
  if (!date) return 'Invalid date';
  return date.toString();
};

const onSubmit = form.handleSubmit(async (values) => {
  try {
    emit('submit', values as AnimeWithoutId)
    toast({
      title: "Éxito",
      description: props.anime ? "El anime ha sido actualizado correctamente." : "El anime ha sido añadido correctamente.",
    })
    if (!props.anime) form.resetForm()
  } catch (error) {
    toast({
      title: "Error",
      description: animeStore.error || "Hubo un error al procesar el anime.",
      variant: "destructive",
    })
  }
})

</script>
<template>
  <form @submit="onSubmit"
        class="space-y-6 max-w-2xl mx-auto p-6 bg-muted/40 rounded-lg shadow text-foreground py-2">
    <h2 class="text-2xl font-bold mb-6 my-2 text-center">
      {{ props.anime ? 'Actualizar Anime' : 'Añadir Nuevo Anime' }}
    </h2>


    <FormField v-slot="{ componentField }" name="title">
      <FormItem>
        <FormLabel>Título</FormLabel>
        <FormControl>
          <Input v-bind="componentField"/>
        </FormControl>
        <FormMessage/>
      </FormItem>
    </FormField>

    <FormField v-slot="{ componentField }" name="description">
      <FormItem>
        <FormLabel>Descripción</FormLabel>
        <FormControl>
          <Textarea v-bind="componentField"/>
        </FormControl>
        <FormMessage/>
      </FormItem>
    </FormField>

    <FormField v-slot="{ componentField }" name="image_path">
      <FormItem>
        <FormLabel>Imagen</FormLabel>
        <FormControl>
          <Button
              type="button"
              @click="handleImageUpload"
              class="flex items-center justify-center gap-2"
          >
            <UploadIcon class="w-5 h-5"/>
            Seleccionar Imagen
          </Button>
        </FormControl>
        <img v-if="form.values.image_path" :src="form.values.image_path" alt="Preview"
             class="mt-2 max-w-xs rounded"/>
        <FormMessage/>
      </FormItem>
    </FormField>

    <FormField v-slot="{ componentField }" name="total_episodes">
      <FormItem>
        <FormLabel>Número total de episodios</FormLabel>
        <FormControl>
          <Input v-bind="componentField" type="number"/>
        </FormControl>
        <FormMessage/>
      </FormItem>
    </FormField>

    <div class="grid grid-cols-2 gap-4">
      <FormField v-slot="{ value }" name="release_date">
        <FormItem>
          <FormLabel>Fecha de estreno</FormLabel>
          <Popover>
            <PopoverTrigger as-child>
              <FormControl>
                <Button variant="outline" class="w-full justify-start text-left font-normal">
                  <CalendarIcon class="mr-2 h-4 w-4"/>
                  {{ value ? formatDate(value) : 'Seleccionar fecha' }}
                </Button>
              </FormControl>
            </PopoverTrigger>
            <PopoverContent class="w-auto p-0">
              <Calendar
                  mode="single"
                  :selected="parseDate(value)"
                  @update:model-value="(date) => handleDateChange('release_date', date)"
                  initialFocus
              />
            </PopoverContent>
          </Popover>
          <FormMessage/>
        </FormItem>
      </FormField>

      <FormField v-slot="{ value }" name="end_date">
        <FormItem>
          <FormLabel>Fecha de finalización</FormLabel>
          <Popover>
            <PopoverTrigger as-child>
              <FormControl>
                <Button variant="outline" class="w-full justify-start text-left font-normal">
                  <CalendarIcon class="mr-2 h-4 w-4"/>
                  {{ value ? formatDate(value) : 'Seleccionar fecha' }}
                </Button>
              </FormControl>
            </PopoverTrigger>
            <PopoverContent class="w-auto p-0">
              <Calendar
                  mode="single"
                  :selected="parseDate(value)"
                  @update:model-value="(date) => handleDateChange('end_date', date)"
                  initialFocus
              />
            </PopoverContent>
          </Popover>
          <FormMessage/>
        </FormItem>
      </FormField>
    </div>

    <FormField v-slot="{ componentField }" name="status">
      <FormItem>
        <FormLabel>Estado</FormLabel>
        <Select v-bind="componentField">
          <FormControl>
            <SelectTrigger>
              <SelectValue placeholder="Seleccionar estado"/>
            </SelectTrigger>
          </FormControl>
          <SelectContent>
            <SelectItem value="En emisión">En emisión</SelectItem>
            <SelectItem value="Terminado">Terminado</SelectItem>
          </SelectContent>
        </Select>
        <FormMessage/>
      </FormItem>
    </FormField>

    <FormField v-slot="{ componentField }" name="user_status">
      <FormItem>
        <FormLabel>Estado del usuario</FormLabel>
        <Select v-bind="componentField">
          <FormControl>
            <SelectTrigger>
              <SelectValue placeholder="Seleccionar estado del usuario"/>
            </SelectTrigger>
          </FormControl>
          <SelectContent>
            <SelectItem value="Visto">Visto</SelectItem>
            <SelectItem value="No visto">No visto</SelectItem>
            <SelectItem value="En progreso">En progreso</SelectItem>
            <SelectItem value="Planificado">Planificado</SelectItem>
          </SelectContent>
        </Select>
        <FormMessage/>
      </FormItem>
    </FormField>

    <FormField v-slot="{ componentField }" name="user_rating">
      <FormItem>
        <FormLabel>Calificación del usuario</FormLabel>
        <FormControl>
          <Input v-bind="componentField" type="number" min="0" max="10" step="0.1"/>
        </FormControl>
        <FormMessage/>
      </FormItem>
    </FormField>


    <Button type="submit" class="w-full">
      {{ props.anime ? 'Actualizar Anime' : 'Añadir Anime' }}
    </Button>
  </form>
</template>