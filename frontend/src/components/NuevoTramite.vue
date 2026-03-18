<script lang="ts">
import { useUsuarioStore } from "@/stores/usuario.store";
import { mapState } from "pinia";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ArrowRightLeft, ArrowUpFromLine } from "lucide-vue-next";

export default {
  components: {
    Button,
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    ArrowRightLeft,
    ArrowUpFromLine,
  },
  computed: {
    ...mapState(useUsuarioStore, ["usuario"]),
    tieneNucleo(): boolean {
      return Boolean(this.usuario?.numero_nucleo);
    },
    tituloTramite(): string {
      return this.tieneNucleo ? "Solicitar baja" : "Solicitar alta";
    },
    descripcionTramite(): string {
      return this.tieneNucleo
        ? "Como tienes un nucleo asignado, puedes iniciar la solicitud de baja."
        : "No tienes un nucleo asignado, puedes iniciar la solicitud de alta.";
    },
  },
  methods: {
    solicitar() {
      // TODO: conectar con el flujo real de creacion de tramites
    },
  },
};
</script>

<template>
  <div class="w-fit p-6">
    <Card>
      <CardHeader>
        <CardTitle>Nuevo tramite</CardTitle>
        <CardDescription>
          Desde aqui puedes iniciar una nueva solicitud.
        </CardDescription>
      </CardHeader>
      <CardContent class="flex flex-col gap-4">
        <div class="rounded-lg border p-4">
          <div class="flex items-start justify-between gap-4">
            <div class="flex flex-col gap-1">
              <div class="flex items-center gap-2">
                <ArrowRightLeft
                  v-if="tieneNucleo"
                  class="size-4 text-muted-foreground"
                />
                <ArrowUpFromLine v-else class="size-4 text-muted-foreground" />
                <p class="text-sm font-semibold">{{ tituloTramite }}</p>
              </div>
              <p class="text-sm text-muted-foreground">
                {{ descripcionTramite }}
              </p>
            </div>
            <Button class="hover:cursor-pointer" @click="solicitar">
              {{ tituloTramite }}
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
