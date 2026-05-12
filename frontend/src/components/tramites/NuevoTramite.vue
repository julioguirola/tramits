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
import TramiteLibreta from "@/components/tramites/TramiteLibreta.vue";
import { ClipboardList, Loader2, XCircle, Mail } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { api } from "@/lib/utils";

export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    TramiteAlta,
    TramiteBaja,
    TramiteLibreta,
    ClipboardList,
    Loader2,
    XCircle,
    Mail,
    Button,
    Input,
    Label,
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
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
    bloqueaLibreta(): boolean {
      return this.tramitesPendientesOEnProceso.some(
        (tramite) => tramite.tipo === "Libreta",
      );
    },
    mensajeBloqueoLibreta(): string {
      return this.bloqueaLibreta
        ? "Ya tienes una solicitud de libreta pendiente o en proceso."
        : "";
    },
    tramiteActivoConRegistrador(): any | null {
      return (
        this.tramitesPendientesOEnProceso.find(
          (tramite) =>
            (tramite.estado === "Pendiente" || tramite.estado === "En proceso") &&
            Boolean(tramite.registrador),
        ) || null
      );
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
    abrirCorreoRegistrador(tramite: any) {
      this.correo_tramite_id = tramite.id;
      this.correo_asunto = "";
      this.correo_cuerpo = "";
      this.dialog_correo_open = true;
    },
    async enviarCorreoRegistrador() {
      if (!this.correo_tramite_id || this.enviando_correo) return;
      const asunto = this.correo_asunto.trim();
      const cuerpo = this.correo_cuerpo.trim();
      if (!asunto || !cuerpo) return;

      this.enviando_correo = true;
      try {
        const res = await api.post(
          `/tramite/${this.correo_tramite_id}/correo-registrador`,
          { asunto, cuerpo },
        );
        if (res?.status === 200) {
          this.dialog_correo_open = false;
        }
      } finally {
        this.enviando_correo = false;
      }
    },
  },
  data(): {
    loading_cancelar: string | null;
    dialog_correo_open: boolean;
    correo_tramite_id: string | null;
    correo_asunto: string;
    correo_cuerpo: string;
    enviando_correo: boolean;
  } {
    return {
      loading_cancelar: null,
      dialog_correo_open: false,
      correo_tramite_id: null,
      correo_asunto: "",
      correo_cuerpo: "",
      enviando_correo: false,
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
              <div v-if="tramite.registrador" class="flex items-center gap-2 text-muted-foreground">
                <span>Registrador:</span>
                <span class="font-medium text-foreground">{{ tramite.registrador }}</span>
              </div>
              <Button
                v-if="tramite.estado === 'Pendiente'"
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
              <Button
                v-if="tramite.registrador"
                size="sm"
                variant="outline"
                class="gap-2"
                @click="abrirCorreoRegistrador(tramite)"
              >
                <Mail class="size-4" />
                Escribir correo
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
    <TramiteLibreta
      :bloqueado="bloqueaLibreta"
      :motivo-bloqueo="mensajeBloqueoLibreta"
      @created="refrescarTramites"
    />
  </div>

  <Dialog v-model:open="dialog_correo_open">
    <DialogContent class="max-w-lg">
      <DialogHeader>
        <DialogTitle>Enviar correo al registrador</DialogTitle>
        <DialogDescription>
          Tu mensaje se enviará al registrador asignado a esta solicitud.
        </DialogDescription>
      </DialogHeader>
      <div class="space-y-4">
        <div class="space-y-2">
          <Label for="correo-asunto">Asunto</Label>
          <Input
            id="correo-asunto"
            v-model="correo_asunto"
            placeholder="Ej. Consulta sobre mi trámite"
          />
        </div>
        <div class="space-y-2">
          <Label for="correo-cuerpo">Cuerpo</Label>
          <textarea
            id="correo-cuerpo"
            v-model="correo_cuerpo"
            class="flex min-h-32 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-xs outline-none transition-[color,box-shadow] placeholder:text-muted-foreground focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50"
            rows="6"
            placeholder="Escribe el contenido del correo"
          ></textarea>
        </div>
      </div>
      <DialogFooter class="gap-2">
        <Button
          variant="outline"
          :disabled="enviando_correo"
          @click="dialog_correo_open = false"
        >
          Cancelar
        </Button>
        <Button
          class="gap-2"
          :disabled="enviando_correo || !correo_asunto.trim() || !correo_cuerpo.trim()"
          @click="enviarCorreoRegistrador"
        >
          <Loader2 v-if="enviando_correo" class="size-4 animate-spin" />
          Enviar
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
