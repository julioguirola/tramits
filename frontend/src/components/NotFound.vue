<template>
  <div class="flex flex-col items-center justify-center min-h-screen">
    <h1 class="text-4xl font-bold mb-4">404</h1>
    <p class="text-lg text-gray-600 mb-8">Página no encontrada</p>
    <RouterLink to="/" class="text-blue-500 hover:underline">
      Volver al inicio
    </RouterLink>
  </div>
</template>

<script lang="ts">
import { RouterLink } from "vue-router";

export default {
  components: {
    RouterLink,
  },
};
</script>
