<script lang="ts">
import { useTramiteStore } from "@/stores/tramite.store";
import { mapState, mapActions } from "pinia";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { ClipboardList, Loader2, PlayCircle } from "lucide-vue-next";

export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
    Badge,
    Button,
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    ClipboardList,
    Loader2,
    PlayCircle,
  },
  computed: {
    ...mapState(useTramiteStore, [
      "tramites",
      "tramites_en_proceso",
      "cargando",
    ]),
    tieneSolicitudes(): boolean {
      return Boolean(this.tramites && this.tramites.length > 0);
    },
    tieneSolicitudesEnProceso(): boolean {
      return Boolean(
        this.tramites_en_proceso && this.tramites_en_proceso.length > 0,
      );
    },
  },
  data(): {
    loading_procesar: string | null;
    dialog_gestion_open: boolean;
    selected_tramite_id: string | null;
    loading_gestion: boolean;
  } {
    return {
      loading_procesar: null,
      dialog_gestion_open: false,
      selected_tramite_id: null,
      loading_gestion: false,
    };
  },
  methods: {
    ...mapActions(useTramiteStore, [
      "cargarHistorial",
      "gestionarTramite",
    ]),
    ...mapActions(useTramiteStore, {
      procesarSolicitudAction: "procesarSolicitud",
    }),
    getEstadoVariant(
      estado: string,
    ): "default" | "secondary" | "outline" | "destructive" {
      if (estado === "Completado") return "default";
      if (estado === "Rechazado") return "destructive";
      if (estado === "En proceso") return "outline";
      return "secondary"; // Pendiente
    },
    getEstadoClasses(estado: string): string {
      if (estado === "En proceso") {
        return "bg-yellow-100 text-yellow-800 hover:bg-yellow-100/80 dark:bg-yellow-900/30 dark:text-yellow-400";
      }
      return "";
    },
    async procesarSolicitud(id: string) {
      if (this.loading_procesar) return;
      this.loading_procesar = id;
      const ok = await this.procesarSolicitudAction(id);
      if (ok) {
        await this.cargarHistorial(1);
        await this.cargarHistorial(2, true);
      }
      this.loading_procesar = null;
    },
    abrirDialogGestion(id: string) {
      this.selected_tramite_id = id;
      this.dialog_gestion_open = true;
    },
    async gestionar(accion: "completar" | "rechazar") {
      if (!this.selected_tramite_id || this.loading_gestion) return;
      this.loading_gestion = true;
      const ok = await this.gestionarTramite(this.selected_tramite_id, accion);
      if (ok) {
        this.dialog_gestion_open = false;
        this.selected_tramite_id = null;
        await this.cargarHistorial(1);
        await this.cargarHistorial(2, true);
      }
      this.loading_gestion = false;
    },
    getNombreCompleto(tramite: any): string {
      if (tramite.persona_nombre && tramite.persona_apellido) {
        return `${tramite.persona_nombre} ${tramite.persona_apellido}`;
      }
      return "-";
    },
  },
  async mounted() {
    // Cargar solo solicitudes pendientes (estado_id = 1)
    await this.cargarHistorial(1);
    await this.cargarHistorial(2, true);
  },
};
</script>

<template>
  <div class="p-6 w-full">
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2">
          <ClipboardList class="size-5" />
          Solicitudes Pendientes
        </CardTitle>
        <CardDescription>
          Gestiona las solicitudes de trámites que están pendientes de procesar.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <!-- Estado de carga -->
        <div v-if="cargando" class="flex items-center justify-center py-8">
          <Loader2 class="size-6 animate-spin text-muted-foreground" />
          <span class="ml-2 text-sm text-muted-foreground"
            >Cargando solicitudes...</span
          >
        </div>

        <template v-else>
          <!-- Sin solicitudes pendientes -->
          <div v-if="!tieneSolicitudes" class="py-8 text-center">
            <ClipboardList
              class="mx-auto size-12 text-muted-foreground opacity-50"
            />
            <p class="mt-4 text-sm text-muted-foreground">
              No hay solicitudes pendientes en este momento.
            </p>
          </div>

          <!-- Con solicitudes pendientes -->
          <div v-else>
            <!-- Tabla con solicitudes - Vista Desktop -->
            <div class="hidden rounded-md border md:block">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Solicitante</TableHead>
                    <TableHead>Tipo</TableHead>
                    <TableHead>Núcleo</TableHead>
                    <TableHead>Fecha Solicitud</TableHead>
                    <TableHead>Estado</TableHead>
                    <TableHead>Acciones</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  <TableRow v-for="tramite in tramites" :key="tramite.id">
                    <TableCell class="font-medium">
                      {{ getNombreCompleto(tramite) }}
                    </TableCell>
                    <TableCell>{{ tramite.tipo }}</TableCell>
                    <TableCell>{{ tramite.nucleo }}</TableCell>
                    <TableCell>{{ tramite.fecha_solicitud }}</TableCell>
                    <TableCell>
                      <Badge
                        :variant="getEstadoVariant(tramite.estado)"
                        :class="getEstadoClasses(tramite.estado)"
                      >
                        {{ tramite.estado }}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      <Button
                        size="sm"
                        @click="procesarSolicitud(tramite.id)"
                        :disabled="Boolean(loading_procesar)"
                        class="gap-2"
                      >
                        <Loader2
                          v-if="loading_procesar === tramite.id"
                          class="size-4 animate-spin"
                        />
                        <PlayCircle v-else class="size-4" />
                        {{
                          loading_procesar === tramite.id
                            ? "Procesando"
                            : "Procesar"
                        }}
                      </Button>
                    </TableCell>
                  </TableRow>
                </TableBody>
              </Table>
            </div>

            <!-- Vista Mobile - Cards apiladas -->
            <div class="space-y-4 md:hidden">
              <div
                v-for="tramite in tramites"
                :key="tramite.id"
                class="rounded-lg border p-4 space-y-3"
              >
                <div class="flex items-center justify-between">
                  <span class="font-semibold text-sm">{{ tramite.tipo }}</span>
                  <Badge
                    :variant="getEstadoVariant(tramite.estado)"
                    :class="getEstadoClasses(tramite.estado)"
                  >
                    {{ tramite.estado }}
                  </Badge>
                </div>
                <div class="space-y-2 text-sm">
                  <div class="flex justify-between">
                    <span class="text-muted-foreground">Solicitante:</span>
                    <span class="font-medium">{{
                      getNombreCompleto(tramite)
                    }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-muted-foreground">Núcleo:</span>
                    <span>{{ tramite.nucleo }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-muted-foreground">Fecha Solicitud:</span>
                    <span>{{ tramite.fecha_solicitud }}</span>
                  </div>
                </div>
                <Button
                  @click="procesarSolicitud(tramite.id)"
                  class="w-full gap-2"
                  size="sm"
                  :disabled="Boolean(loading_procesar)"
                >
                  <Loader2
                    v-if="loading_procesar === tramite.id"
                    class="size-4 animate-spin"
                  />
                  <PlayCircle v-else class="size-4" />
                  {{
                    loading_procesar === tramite.id ? "Procesando" : "Procesar"
                  }}
                </Button>
              </div>
            </div>
          </div>

          <div class="mt-8">
            <h3 class="mb-4 text-base font-semibold text-foreground">
              Mis solicitudes en proceso
            </h3>

            <div
              v-if="!tieneSolicitudesEnProceso"
              class="rounded-md border py-6 text-center"
            >
              <p class="text-sm text-muted-foreground">
                No tienes solicitudes en proceso asignadas actualmente.
              </p>
            </div>

            <div v-else>
              <div class="hidden rounded-md border md:block">
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead>Solicitante</TableHead>
                      <TableHead>Tipo</TableHead>
                      <TableHead>Núcleo</TableHead>
                      <TableHead>Fecha Solicitud</TableHead>
                      <TableHead>Estado</TableHead>
                      <TableHead>Registrador</TableHead>
                      <TableHead>Acciones</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    <TableRow
                      v-for="tramite in tramites_en_proceso"
                      :key="tramite.id"
                    >
                      <TableCell class="font-medium">
                        {{ getNombreCompleto(tramite) }}
                      </TableCell>
                      <TableCell>{{ tramite.tipo }}</TableCell>
                      <TableCell>{{ tramite.nucleo }}</TableCell>
                      <TableCell>{{ tramite.fecha_solicitud }}</TableCell>
                      <TableCell>
                        <Badge
                          :variant="getEstadoVariant(tramite.estado)"
                          :class="getEstadoClasses(tramite.estado)"
                        >
                          {{ tramite.estado }}
                        </Badge>
                      </TableCell>
                      <TableCell>{{ tramite.registrador || "-" }}</TableCell>
                      <TableCell>
                        <Button
                          size="sm"
                          @click="abrirDialogGestion(tramite.id)"
                        >
                          Gestionar
                        </Button>
                      </TableCell>
                    </TableRow>
                  </TableBody>
                </Table>
              </div>

              <div class="space-y-4 md:hidden">
                <div
                  v-for="tramite in tramites_en_proceso"
                  :key="tramite.id"
                  class="rounded-lg border p-4 space-y-3"
                >
                  <div class="flex items-center justify-between">
                    <span class="font-semibold text-sm">{{
                      tramite.tipo
                    }}</span>
                    <Badge
                      :variant="getEstadoVariant(tramite.estado)"
                      :class="getEstadoClasses(tramite.estado)"
                    >
                      {{ tramite.estado }}
                    </Badge>
                  </div>
                  <div class="space-y-2 text-sm">
                    <div class="flex justify-between">
                      <span class="text-muted-foreground">Solicitante:</span>
                      <span class="font-medium">{{
                        getNombreCompleto(tramite)
                      }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-muted-foreground">Núcleo:</span>
                      <span>{{ tramite.nucleo }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-muted-foreground"
                        >Fecha Solicitud:</span
                      >
                      <span>{{ tramite.fecha_solicitud }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-muted-foreground">Registrador:</span>
                      <span>{{ tramite.registrador || "-" }}</span>
                    </div>
                  </div>
                  <Button
                    class="w-full"
                    size="sm"
                    variant="outline"
                    @click="abrirDialogGestion(tramite.id)"
                  >
                    Gestionar
                  </Button>
                </div>
              </div>
            </div>
          </div>
          <Dialog v-model:open="dialog_gestion_open">
            <DialogContent class="sm:max-w-md">
              <DialogHeader>
                <DialogTitle>Gestionar Solicitud</DialogTitle>
                <DialogDescription>
                  Selecciona si deseas rechazar o completar este trámite.
                </DialogDescription>
              </DialogHeader>
              <DialogFooter>
                <Button
                  variant="destructive"
                  :disabled="loading_gestion"
                  @click="gestionar('rechazar')"
                >
                  <Loader2 v-if="loading_gestion" class="size-4 animate-spin" />
                  {{ loading_gestion ? "" : "Rechazar" }}
                </Button>
                <Button
                  :disabled="loading_gestion"
                  @click="gestionar('completar')"
                >
                  <Loader2 v-if="loading_gestion" class="size-4 animate-spin" />
                  {{ loading_gestion ? "" : "Completar trámite" }}
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </template>
      </CardContent>
    </Card>
  </div>
</template>
