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
import { ClipboardList, Loader2, PlayCircle, Download } from "lucide-vue-next";
import * as XLSX from "xlsx";
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationFirst,
  PaginationItem,
  PaginationLast,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

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
    Pagination,
    PaginationContent,
    PaginationEllipsis,
    PaginationFirst,
    PaginationItem,
    PaginationLast,
    PaginationNext,
    PaginationPrevious,
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectTrigger,
    SelectValue,
    Download,
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
    totalPagesPendientes(): number {
      return Math.max(
        1,
        Math.ceil(this.total_pendientes / this.limit_pendientes),
      );
    },
    totalPagesEnProceso(): number {
      return Math.max(
        1,
        Math.ceil(this.total_en_proceso / this.limit_en_proceso),
      );
    },
    tramitesPendientesPaginados(): any[] {
      if (!this.tramites) return [];
      const start = (this.page_pendientes - 1) * this.limit_pendientes;
      return this.tramites.slice(start, start + this.limit_pendientes);
    },
    tramitesEnProcesoPaginados(): any[] {
      if (!this.tramites_en_proceso) return [];
      const start = (this.page_en_proceso - 1) * this.limit_en_proceso;
      return this.tramites_en_proceso.slice(
        start,
        start + this.limit_en_proceso,
      );
    },
  },
  data(): {
    loading_procesar: string | null;
    dialog_gestion_open: boolean;
    selected_tramite_id: string | null;
    loading_gestion: boolean;
    dialog_procesar_open: boolean;
    selected_procesar_id: string | null;
    page_pendientes: number;
    limit_pendientes: number;
    total_pendientes: number;
    page_en_proceso: number;
    limit_en_proceso: number;
    total_en_proceso: number;
    limitOptions: number[];
  } {
    return {
      loading_procesar: null,
      dialog_gestion_open: false,
      selected_tramite_id: null,
      loading_gestion: false,
      dialog_procesar_open: false,
      selected_procesar_id: null,
      page_pendientes: 1,
      limit_pendientes: 10,
      total_pendientes: 0,
      page_en_proceso: 1,
      limit_en_proceso: 10,
      total_en_proceso: 0,
      limitOptions: [5, 10, 20, 30],
    };
  },
  methods: {
    ...mapActions(useTramiteStore, ["cargarHistorial", "gestionarTramite"]),
    async cargarPendientes() {
      await this.cargarHistorial(1);
      if (this.tramites) {
        this.total_pendientes = this.tramites.length;
      }
    },
    async cargarEnProceso() {
      await this.cargarHistorial(2, true);
      if (this.tramites_en_proceso) {
        this.total_en_proceso = this.tramites_en_proceso.length;
      }
    },
    async irAPaginaPendientes(page: number) {
      if (this.cargando || page === this.page_pendientes || page < 1) return;
      this.page_pendientes = page;
    },
    async irAPaginaEnProceso(page: number) {
      if (this.cargando || page === this.page_en_proceso || page < 1) return;
      this.page_en_proceso = page;
    },
    async cambiarLimitePendientes(limit: number) {
      if (this.cargando || this.limit_pendientes === limit) return;
      this.limit_pendientes = limit;
      this.page_pendientes = 1;
    },
    async cambiarLimiteEnProceso(limit: number) {
      if (this.cargando || this.limit_en_proceso === limit) return;
      this.limit_en_proceso = limit;
      this.page_en_proceso = 1;
    },
    exportarPendientes() {
      if (
        !this.tramitesPendientesPaginados ||
        this.tramitesPendientesPaginados.length === 0
      )
        return;
      const data = this.tramitesPendientesPaginados.map((t) => ({
        Solicitante: this.getNombreCompleto(t),
        Tipo: t.tipo,
        Núcleo: t.nucleo,
        "Fecha de Solicitud": t.fecha_solicitud,
        Estado: t.estado,
      }));
      const ws = XLSX.utils.json_to_sheet(data);
      const wb = XLSX.utils.book_new();
      XLSX.utils.book_append_sheet(wb, ws, "Pendientes");
      XLSX.writeFile(
        wb,
        `solicitudes_pendientes_${new Date().toISOString().split("T")[0]}.xlsx`,
      );
    },
    exportarEnProceso() {
      if (
        !this.tramitesEnProcesoPaginados ||
        this.tramitesEnProcesoPaginados.length === 0
      )
        return;
      const data = this.tramitesEnProcesoPaginados.map((t) => ({
        Solicitante: this.getNombreCompleto(t),
        Tipo: t.tipo,
        Núcleo: t.nucleo,
        "Fecha de Solicitud": t.fecha_solicitud,
        Estado: t.estado,
        Registrador: t.registrador || "-",
      }));
      const ws = XLSX.utils.json_to_sheet(data);
      const wb = XLSX.utils.book_new();
      XLSX.utils.book_append_sheet(wb, ws, "En Proceso");
      XLSX.writeFile(
        wb,
        `solicitudes_en_proceso_${new Date().toISOString().split("T")[0]}.xlsx`,
      );
    },
    ...mapActions(useTramiteStore, {
      procesarSolicitudAction: "procesarSolicitud",
    }),
    getEstadoVariant(
      estado: string,
    ): "default" | "secondary" | "outline" | "destructive" {
      if (estado === "Completado") return "default";
      if (estado === "Rechazado") return "destructive";
      if (estado === "Cancelado") return "outline";
      if (estado === "En proceso") return "outline";
      return "secondary"; // Pendiente
    },
    getEstadoClasses(estado: string): string {
      if (estado === "En proceso") {
        return "bg-yellow-100 text-yellow-800 hover:bg-yellow-100/80 dark:bg-yellow-900/30 dark:text-yellow-400";
      }
      if (estado === "Cancelado") {
        return "bg-slate-100 text-slate-700 hover:bg-slate-100/80 dark:bg-slate-900/30 dark:text-slate-300";
      }
      return "";
    },
    async procesarSolicitud(id: string) {
      if (this.loading_procesar) return;
      this.loading_procesar = id;
      const ok = await this.procesarSolicitudAction(id);
      if (ok) {
        await this.cargarPendientes();
        await this.cargarEnProceso();
      }
      this.loading_procesar = null;
    },
    abrirDialogProcesar(id: string) {
      this.selected_procesar_id = id;
      this.dialog_procesar_open = true;
    },
    async confirmarProcesar() {
      if (!this.selected_procesar_id || this.loading_procesar) return;
      await this.procesarSolicitud(this.selected_procesar_id);
      this.dialog_procesar_open = false;
      this.selected_procesar_id = null;
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
        await this.cargarPendientes();
        await this.cargarEnProceso();
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
    await this.cargarPendientes();
    await this.cargarEnProceso();
  },
};
</script>

<template>
  <div class="p-6 w-full">
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center justify-between">
          <span class="flex items-center gap-2">
            <ClipboardList class="size-5" />
            Solicitudes Pendientes
          </span>
          <Button
            v-if="tieneSolicitudes"
            size="sm"
            variant="outline"
            class="gap-2"
            @click="exportarPendientes"
          >
            <Download class="size-4" />
            Exportar
          </Button>
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
                    <TableHead>Fecha de solicitud</TableHead>
                    <TableHead>Estado</TableHead>
                    <TableHead>Acciones</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  <TableRow
                    v-for="tramite in tramitesPendientesPaginados"
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
                    <TableCell>
                      <Button
                        size="sm"
                        @click="abrirDialogProcesar(tramite.id)"
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
                v-for="tramite in tramitesPendientesPaginados"
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
                    <span class="text-muted-foreground"
                      >Fecha de solicitud:</span
                    >
                    <span>{{ tramite.fecha_solicitud }}</span>
                  </div>
                </div>
                <Button
                  @click="abrirDialogProcesar(tramite.id)"
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
            <div
              class="mt-6 flex flex-col gap-4 border-t pt-4 sm:flex-row sm:items-center sm:justify-between"
            >
              <div
                class="flex flex-wrap items-center gap-3 text-sm text-muted-foreground"
              >
                <span
                  >Página {{ page_pendientes }} de
                  {{ totalPagesPendientes }}</span
                >
                <span>({{ total_pendientes }} trámites)</span>
              </div>
              <div class="flex flex-wrap items-center gap-4">
                <div class="flex items-center gap-2 text-sm">
                  <span class="text-muted-foreground">Tamaño de página</span>
                  <Select
                    :model-value="String(limit_pendientes)"
                    @update:model-value="
                      (value) => cambiarLimitePendientes(Number(value))
                    "
                  >
                    <SelectTrigger class="h-9 w-30]">
                      <SelectValue placeholder="Selecciona" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectGroup>
                        <SelectLabel>Resultados</SelectLabel>
                        <SelectItem
                          v-for="option in limitOptions"
                          :key="option"
                          :value="String(option)"
                        >
                          {{ option }}
                        </SelectItem>
                      </SelectGroup>
                    </SelectContent>
                  </Select>
                </div>
                <Pagination
                  :page="page_pendientes"
                  :total="total_pendientes"
                  :sibling-count="1"
                  :items-per-page="limit_pendientes"
                  @update:page="irAPaginaPendientes"
                >
                  <PaginationContent v-slot="{ items }">
                    <PaginationFirst />
                    <PaginationPrevious />
                    <template
                      v-for="(item, index) in items"
                      :key="item.type + '-' + index"
                    >
                      <PaginationItem
                        v-if="item.type === 'page'"
                        :value="item.value"
                        :is-active="item.value === page_pendientes"
                      >
                        {{ item.value }}
                      </PaginationItem>
                      <PaginationEllipsis
                        v-else-if="item.type === 'ellipsis'"
                      />
                    </template>
                    <PaginationNext />
                    <PaginationLast />
                  </PaginationContent>
                </Pagination>
              </div>
            </div>
          </div>

          <div class="mt-8">
            <div class="mb-4 flex items-center justify-between">
              <h3 class="text-base font-semibold text-foreground">
                Mis solicitudes en proceso
              </h3>
              <Button
                v-if="tieneSolicitudesEnProceso"
                size="sm"
                variant="outline"
                class="gap-2"
                @click="exportarEnProceso"
              >
                <Download class="size-4" />
                Exportar
              </Button>
            </div>

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
                      <TableHead>Fecha de solicitud</TableHead>
                      <TableHead>Estado</TableHead>
                      <TableHead>Registrador</TableHead>
                      <TableHead>Acciones</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    <TableRow
                      v-for="tramite in tramitesEnProcesoPaginados"
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
                  v-for="tramite in tramitesEnProcesoPaginados"
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
                        >Fecha de solicitud:</span
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
              <div
                class="mt-6 flex flex-col gap-4 border-t pt-4 sm:flex-row sm:items-center sm:justify-between"
              >
                <div
                  class="flex flex-wrap items-center gap-3 text-sm text-muted-foreground"
                >
                  <span
                    >Página {{ page_en_proceso }} de
                    {{ totalPagesEnProceso }}</span
                  >
                  <span>({{ total_en_proceso }} trámites)</span>
                </div>
                <div class="flex flex-wrap items-center gap-4">
                  <div class="flex items-center gap-2 text-sm">
                    <span class="text-muted-foreground">Tamaño de página</span>
                    <Select
                      :model-value="String(limit_en_proceso)"
                      @update:model-value="
                        (value) => cambiarLimiteEnProceso(Number(value))
                      "
                    >
                      <SelectTrigger class="h-9 w-30">
                        <SelectValue placeholder="Selecciona" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectGroup>
                          <SelectLabel>Resultados</SelectLabel>
                          <SelectItem
                            v-for="option in limitOptions"
                            :key="option"
                            :value="String(option)"
                          >
                            {{ option }}
                          </SelectItem>
                        </SelectGroup>
                      </SelectContent>
                    </Select>
                  </div>
                  <Pagination
                    :page="page_en_proceso"
                    :total="total_en_proceso"
                    :sibling-count="1"
                    :items-per-page="limit_en_proceso"
                    @update:page="irAPaginaEnProceso"
                  >
                    <PaginationContent v-slot="{ items }">
                      <PaginationFirst />
                      <PaginationPrevious />
                      <template
                        v-for="(item, index) in items"
                        :key="item.type + '-' + index"
                      >
                        <PaginationItem
                          v-if="item.type === 'page'"
                          :value="item.value"
                          :is-active="item.value === page_en_proceso"
                        >
                          {{ item.value }}
                        </PaginationItem>
                        <PaginationEllipsis
                          v-else-if="item.type === 'ellipsis'"
                        />
                      </template>
                      <PaginationNext />
                      <PaginationLast />
                    </PaginationContent>
                  </Pagination>
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
          <Dialog v-model:open="dialog_procesar_open">
            <DialogContent class="sm:max-w-md">
              <DialogHeader>
                <DialogTitle>Procesar solicitud</DialogTitle>
                <DialogDescription>
                  Al procesar esta solicitud, asumirás la responsabilidad de
                  gestionarla y mantener la comunicación con el consumidor que
                  la solicitó.
                </DialogDescription>
              </DialogHeader>
              <DialogFooter>
                <Button
                  variant="outline"
                  :disabled="loading_procesar"
                  @click="dialog_procesar_open = false"
                >
                  Volver
                </Button>
                <Button :disabled="loading_procesar" @click="confirmarProcesar">
                  <Loader2
                    v-if="loading_procesar"
                    class="size-4 animate-spin"
                  />
                  {{ loading_procesar ? "" : "Confirmar y procesar" }}
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </template>
      </CardContent>
    </Card>
  </div>
</template>
