<script lang="ts">
import { useUsuarioStore } from "@/stores/usuario.store";
import { mapState } from "pinia";

const mensajes: Record<string, string> = {
  Consumidor:
    "Accede al menú lateral para solicitar el alta o baja de un núcleo familiar.",
  Registrador:
    "Accede al menú lateral para revisar y gestionar las solicitudes de trámites pendientes.",
  Administrador:
    "Accede al menú lateral para administrar los usuarios del sistema.",
};

export default {
  computed: {
    ...mapState(useUsuarioStore, ["usuario"]),
    mensaje(): string {
      return this.usuario?.tipo
        ? (mensajes[this.usuario.tipo] ??
            "Selecciona una opción del menú lateral.")
        : "";
    },
  },
};
</script>

<template>
  <div
    class="flex flex-col items-center justify-center h-full gap-8 px-8 py-12 text-center"
  >
    <img
      src="/tramites.webp"
      alt="Trámites"
      class="w-full max-w-md select-none pointer-events-none"
    />
    <div class="flex flex-col gap-2">
      <h1 class="text-2xl font-semibold tracking-tight">
        Bienvenido{{ usuario ? `, ${usuario.nombre}` : "" }}
      </h1>
      <p class="text-sm text-muted-foreground max-w-sm">
        {{ mensaje }}
      </p>
    </div>
  </div>
</template>
