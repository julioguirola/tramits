<script lang="ts">
import { useUsuarioStore } from "@/stores/usuario.store";
import { mapState } from "pinia";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import TramiteAlta from "@/components/TramiteAlta.vue";
import TramiteBaja from "@/components/TramiteBaja.vue";

export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    TramiteAlta,
    TramiteBaja,
  },
  computed: {
    ...mapState(useUsuarioStore, ["usuario"]),
    tieneNucleo(): boolean {
      return Boolean(this.usuario?.numero_nucleo);
    },
  },
};
</script>

<template>
  <div class="w-full md:w-2/3 p-6 space-y-4">
    <Card>
      <CardHeader>
        <CardTitle>Nuevo trámite</CardTitle>
        <CardDescription>
          Desde aquí puedes iniciar una nueva solicitud.
        </CardDescription>
      </CardHeader>
    </Card>

    <TramiteAlta />
    <TramiteBaja />
  </div>
</template>
