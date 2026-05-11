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
import {
  Home,
  Store,
  Building2,
  MapPin,
  Map,
  Mail,
  Loader2,
} from "lucide-vue-next";
import HistorialTramites from "@/components/tramites/HistorialTramites.vue";
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
import { useTramiteStore } from "@/stores/tramite.store";

export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    Button,
    Input,
    Label,
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    HistorialTramites,
    Mail,
    Loader2,
  },
  data() {
    return {
      icons: { Home, Store, Building2, MapPin, Map },
      dialogCorreoAbierto: false,
      correoAsunto: "",
      correoCuerpo: "",
      enviandoCorreo: false,
    };
  },
  computed: {
    ...mapState(useUsuarioStore, ["usuario"]),
    ...mapState(useTramiteStore, ["tramites", "cargando"]),
    campos(): { label: string; valor: string | null; icon: object }[] {
      if (!this.usuario) return [];
      const nucleoLabel =
        this.usuario.nucleo && this.usuario.numero_nucleo
          ? `${this.usuario.nucleo} # ${this.usuario.numero_nucleo}`
          : "Sin núcleo asignado";
      return [
        {
          label: "Núcleo",
          valor: nucleoLabel,
          icon: this.icons.Home,
        },
        { label: "Bodega", valor: this.usuario.bodega, icon: this.icons.Store },
        {
          label: "Oficina",
          valor: this.usuario.oficina,
          icon: this.icons.Building2,
        },
        {
          label: "Municipio",
          valor: this.usuario.municipio,
          icon: this.icons.MapPin,
        },
        {
          label: "Provincia",
          valor: this.usuario.provincia,
          icon: this.icons.Map,
        },
      ];
    },
    tramiteActivo(): any | null {
      if (!this.tramites) return null;
      return (
        this.tramites.find(
          (tramite: any) =>
            tramite.estado === "Pendiente" || tramite.estado === "En proceso",
        ) || null
      );
    },
    registradorAsignado(): boolean {
      return Boolean(this.tramiteActivo?.registrador);
    },
  },
  methods: {
    async cargarTramites() {
      const store = useTramiteStore();
      await store.cargarHistorial();
    },
    abrirCorreo() {
      this.correoAsunto = "";
      this.correoCuerpo = "";
      this.dialogCorreoAbierto = true;
    },
    async enviarCorreo() {
      if (!this.tramiteActivo || this.enviandoCorreo) return;
      const asunto = this.correoAsunto.trim();
      const cuerpo = this.correoCuerpo.trim();
      if (!asunto || !cuerpo) return;

      this.enviandoCorreo = true;
      try {
        const store = useTramiteStore();
        const ok = await store.enviarCorreoRegistrador(
          this.tramiteActivo.id,
          asunto,
          cuerpo,
        );
        if (ok) {
          this.dialogCorreoAbierto = false;
        }
      } finally {
        this.enviandoCorreo = false;
      }
    },
  },
  async mounted() {
    await this.cargarTramites();
  },
};
</script>

<template>
  <div class="flex flex-col gap-6 p-6 w-fit md:self-start self-center">
    <Card>
      <CardHeader>
        <CardTitle>Mi información</CardTitle>
        <CardDescription>
          Datos del núcleo familiar y ubicación asociados a tu cuenta.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="flex flex-col md:flex-row gap-4">
          <div
            v-for="item in campos"
            :key="item.label"
            class="flex items-center gap-3 rounded-lg border p-3"
          >
            <component
              :is="item.icon"
              class="size-5 shrink-0 text-muted-foreground"
            />
            <div class="flex flex-col">
              <span class="text-xs text-muted-foreground">{{
                item.label
              }}</span>
              <span class="text-sm font-medium">{{
                item.valor ?? "Sin asignar"
              }}</span>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>

    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2">
          <Mail class="size-4 text-muted-foreground" />
          Registrador asignado
        </CardTitle>
        <CardDescription>
          Información del registrador a cargo de tu solicitud activa.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div v-if="cargando" class="flex items-center gap-2 text-sm">
          <Loader2 class="size-4 animate-spin" />
          Cargando información...
        </div>
        <div v-else-if="!tramiteActivo" class="text-sm text-muted-foreground">
          No tienes solicitudes activas.
        </div>
        <div v-else class="flex flex-wrap items-center justify-between gap-3">
          <div>
            <p class="text-sm text-muted-foreground">Registrador</p>
            <p class="text-base font-medium">
              {{ tramiteActivo.registrador || "Sin asignar" }}
            </p>
          </div>
          <Button
            size="sm"
            class="gap-2"
            :disabled="!registradorAsignado"
            @click="abrirCorreo"
          >
            <Mail class="size-4" />
            Escribir correo
          </Button>
        </div>
      </CardContent>
    </Card>

    <!-- Historial de trámites -->
    <HistorialTramites />
  </div>

  <Dialog v-model:open="dialogCorreoAbierto">
    <DialogContent class="max-w-lg">
      <DialogHeader>
        <DialogTitle>Enviar correo al registrador</DialogTitle>
        <DialogDescription>
          Tu mensaje se enviará al registrador asignado a la solicitud activa.
        </DialogDescription>
      </DialogHeader>
      <div class="space-y-4">
        <div class="space-y-2">
          <Label for="correo-asunto">Asunto</Label>
          <Input
            id="correo-asunto"
            v-model="correoAsunto"
            placeholder="Ej. Consulta sobre mi trámite"
          />
        </div>
        <div class="space-y-2">
          <Label for="correo-cuerpo">Cuerpo</Label>
          <textarea
            id="correo-cuerpo"
            v-model="correoCuerpo"
            class="flex min-h-32 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-xs outline-none transition-[color,box-shadow] placeholder:text-muted-foreground focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50"
            rows="6"
            placeholder="Escribe el contenido del correo"
          ></textarea>
        </div>
      </div>
      <DialogFooter class="gap-2">
        <Button
          variant="outline"
          :disabled="enviandoCorreo"
          @click="dialogCorreoAbierto = false"
        >
          Cancelar
        </Button>
        <Button
          class="gap-2"
          :disabled="
            enviandoCorreo || !correoAsunto.trim() || !correoCuerpo.trim()
          "
          @click="enviarCorreo"
        >
          <Loader2 v-if="enviandoCorreo" class="size-4 animate-spin" />
          Enviar
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
