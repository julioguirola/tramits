<script lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  },
  props: {
    persona: {
      type: Object,
      required: true,
    },
  },
};
</script>

<template>
  <Card class="flex flex-col gap-4">
    <CardHeader>
      <CardTitle>Información de Persona</CardTitle>
      <CardDescription>Datos verificados del carnet</CardDescription>
    </CardHeader>
    <CardContent>
      <div class="flex flex-col gap-2">
        <div>
          <p class="font-medium text-gray-600">Nombre</p>
          <p>{{ persona.nombre }}</p>
        </div>
        <div>
          <p class="font-medium text-gray-600">Apellido</p>
          <p>{{ persona.apellido }}</p>
        </div>
        <div>
          <p class="font-medium text-gray-600">Carnet</p>
          <p>{{ persona.carnet }}</p>
        </div>
        <div>
          <p class="font-medium text-gray-600">Dirección</p>
          <p>{{ persona.direccion }}</p>
        </div>
      </div>
    </CardContent>
  </Card>
</template>
