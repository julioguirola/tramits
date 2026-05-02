<script lang="ts">
import { useUsuarioStore } from "@/stores/usuario.store";
import { useTramiteStore } from "@/stores/tramite.store";
import { mapActions, mapState } from "pinia";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import TramiteAlta from "@/components/tramites/TramiteAlta.vue";
import TramiteBaja from "@/components/tramites/TramiteBaja.vue";
import { ClipboardList, Loader2, XCircle } from "lucide-vue-next";
import { Button } from "@/components/ui/button";

export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    TramiteAlta,
    TramiteBaja,
    ClipboardList,
    Loader2,
    XCircle,
    Button,
  },
  computed: {
    ...mapState(useUsuarioStore, ["usuario"]),
    ...mapState(useTramiteStore, ["tramites", "cargando"]),
    tramitesPendientesOEnProceso(): any[] {
      return (this.tramites || []).filter(
        (tramite) =>
          tramite.estado === "Pendiente" || tramite.estado === "En proceso",
      );
    },
    bloqueaAlta(): boolean {
      return (
        Boolean(this.usuario?.numero_nucleo) ||
        this.tramitesPendientesOEnProceso.some(
          (tramite) => tramite.tipo === "Alta",
        )
      );
    },
    bloqueaBaja(): boolean {
      return (
        !this.usuario?.numero_nucleo ||
        this.tramitesPendientesOEnProceso.some(
          (tramite) => tramite.tipo === "Baja",
        )
      );
    },
    mensajeBloqueoAlta(): string {
      if (this.usuario?.numero_nucleo) {
        return "Ya tienes un núcleo asignado.";
      }
      return this.bloqueaAlta
        ? "Ya tienes una solicitud de alta pendiente o en proceso."
        : "";
    },
    mensajeBloqueoBaja(): string {
      if (!this.usuario?.numero_nucleo) {
        return "No tienes un núcleo asignado.";
      }
      return this.bloqueaBaja
        ? "Ya tienes una solicitud de baja pendiente o en proceso."
        : "";
    },
  },
  methods: {
    ...mapActions(useTramiteStore, ["cargarHistorial", "cancelarTramite"]),
    async refrescarTramites() {
      await this.cargarHistorial();
    },
    async cancelarSolicitud(id: string) {
      if (this.loading_cancelar) return;
      this.loading_cancelar = id;
      const ok = await this.cancelarTramite(id);
      if (ok) {
        await this.cargarHistorial();
      }
      this.loading_cancelar = null;
    },
  },
  data(): {
    loading_cancelar: string | null;
  } {
    return {
      loading_cancelar: null,
    };
  },
  async mounted() {
    await this.cargarHistorial();
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

    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2">
          <ClipboardList class="size-4 text-muted-foreground" />
          Trámites activos
        </CardTitle>
        <CardDescription>
          Solicitudes pendientes o en proceso asociadas a tu cuenta.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div v-if="cargando" class="flex items-center gap-2 text-sm">
          <Loader2 class="size-4 animate-spin" />
          Cargando trámites...
        </div>
        <div
          v-else-if="tramitesPendientesOEnProceso.length === 0"
          class="text-sm text-muted-foreground"
        >
          No tienes trámites activos en este momento.
        </div>
        <div v-else class="space-y-2">
          <div
            v-for="tramite in tramitesPendientesOEnProceso"
            :key="tramite.id"
            class="flex flex-wrap items-center justify-between gap-2 rounded-lg border p-3 text-sm"
          >
            <div>
              <p class="font-medium">{{ tramite.tipo }}</p>
              <p class="text-muted-foreground">
                Estado: {{ tramite.estado }} · Fecha:
                {{ tramite.fecha_solicitud }}
              </p>
            </div>
            <div class="flex items-center gap-3">
              <p class="text-muted-foreground">Núcleo: {{ tramite.nucleo }}</p>
              <Button
                size="sm"
                variant="destructive"
                class="gap-2"
                :disabled="loading_cancelar === tramite.id"
                @click="cancelarSolicitud(tramite.id)"
              >
                <Loader2
                  v-if="loading_cancelar === tramite.id"
                  class="size-4 animate-spin"
                />
                <XCircle v-else class="size-4" />
                {{
                  loading_cancelar === tramite.id ? "Cancelando" : "Cancelar"
                }}
              </Button>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <TramiteAlta
      :bloqueado="bloqueaAlta"
      :motivo-bloqueo="mensajeBloqueoAlta"
      @created="refrescarTramites"
    />
    <TramiteBaja
      :bloqueado="bloqueaBaja"
      :motivo-bloqueo="mensajeBloqueoBaja"
      @created="refrescarTramites"
    />
  </div>
</template>
