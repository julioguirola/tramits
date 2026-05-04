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
import { ClipboardList, Loader2, XCircle, Download } from "lucide-vue-next";
import * as XLSX from "xlsx";
import { Button } from "@/components/ui/button";
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
    ClipboardList,
    Loader2,
    XCircle,
    Button,
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
    ...mapState(useTramiteStore, ["tramites", "cargando"]),
    tieneTramites(): boolean {
      return Boolean(this.tramites && this.tramites.length > 0);
    },
    totalPages(): number {
      return Math.max(1, Math.ceil(this.total / this.limit));
    },
  },
  data(): {
    loading_cancelar: string | null;
    page: number;
    limit: number;
    total: number;
    limitOptions: number[];
  } {
    return {
      loading_cancelar: null,
      page: 1,
      limit: 10,
      total: 0,
      limitOptions: [5, 10, 20, 30],
    };
  },
  methods: {
    ...mapActions(useTramiteStore, {
      cargarHistorialAction: "cargarHistorial",
      cancelarTramite: "cancelarTramite",
    }),
    async irAPagina(page: number) {
      if (this.cargando || page === this.page || page < 1) return;
      this.page = page;
      const data = await this.cargarHistorialAction(
        undefined,
        undefined,
        page,
        this.limit,
      );
      if (data?.total !== undefined) {
        this.total = data.total;
      }
    },
    async cambiarLimite(limite: number) {
      if (this.cargando || this.limit === limite) return;
      this.limit = limite;
      this.page = 1;
      const data = await this.cargarHistorialAction(
        undefined,
        undefined,
        1,
        limite,
      );
      if (data?.total !== undefined) {
        this.total = data.total;
      }
    },
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
    exportarExcel() {
      if (!this.tramites || this.tramites.length === 0) return;
      const data = this.tramites.map((t) => ({
        Tipo: t.tipo,
        Núcleo: t.nucleo,
        "Fecha de Solicitud": t.fecha_solicitud,
        "Fecha de Finalización": t.fecha_finalizado || "-",
        Registrador: t.registrador || "-",
        Estado: t.estado,
      }));
      const ws = XLSX.utils.json_to_sheet(data);
      const wb = XLSX.utils.book_new();
      XLSX.utils.book_append_sheet(wb, ws, "Historial");
      XLSX.writeFile(
        wb,
        `historial_tramites_${new Date().toISOString().split("T")[0]}.xlsx`,
      );
    },
    async cancelarSolicitud(id: string) {
      if (this.loading_cancelar) return;
      this.loading_cancelar = id;
      const ok = await this.cancelarTramite(id);
      if (ok) {
        const data = await this.cargarHistorialAction(
          undefined,
          undefined,
          this.page,
          this.limit,
        );
        if (data?.total !== undefined) {
          this.total = data.total;
        }
      }
      this.loading_cancelar = null;
    },
  },
  async mounted() {
    const data = await this.cargarHistorialAction(
      undefined,
      undefined,
      this.page,
      this.limit,
    );
    if (data?.total !== undefined) {
      this.total = data.total;
    }
  },
};
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle class="flex items-center justify-between">
        <span class="flex items-center gap-2">
          <ClipboardList class="size-5" />
          Historial de trámites
        </span>
        <Button
          v-if="tieneTramites"
          size="sm"
          variant="outline"
          class="gap-2"
          @click="exportarExcel"
        >
          <Download class="size-4" />
          Exportar
        </Button>
      </CardTitle>
      <CardDescription>
        Consulta el estado de todas tus solicitudes de trámites.
      </CardDescription>
    </CardHeader>
    <CardContent>
      <!-- Estado de carga -->
      <div v-if="cargando" class="flex items-center justify-center py-8">
        <Loader2 class="size-6 animate-spin text-muted-foreground" />
        <span class="ml-2 text-sm text-muted-foreground"
          >Cargando historial...</span
        >
      </div>

      <!-- Sin trámites -->
      <div v-else-if="!tieneTramites" class="py-8 text-center">
        <ClipboardList
          class="mx-auto size-12 text-muted-foreground opacity-50"
        />
        <p class="mt-4 text-sm text-muted-foreground">
          No tienes trámites registrados aún.
        </p>
      </div>

      <!-- Tabla con trámites - Vista Desktop y Mobile -->
      <template v-else>
        <div class="hidden rounded-md border md:block">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Tipo</TableHead>
                <TableHead>Núcleo</TableHead>
                <TableHead>Fecha de solicitud</TableHead>
                <TableHead>Fecha de finalización</TableHead>
                <TableHead>Registrador</TableHead>
                <TableHead>Estado</TableHead>
                <TableHead>Acciones</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow v-for="tramite in tramites" :key="tramite.id">
                <TableCell class="font-medium">{{ tramite.tipo }}</TableCell>
                <TableCell>{{ tramite.nucleo }}</TableCell>
                <TableCell>{{ tramite.fecha_solicitud }}</TableCell>
                <TableCell>
                  {{ tramite.fecha_finalizado || "-" }}
                </TableCell>
                <TableCell>
                  {{ tramite.registrador || "-" }}
                </TableCell>
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
                      loading_cancelar === tramite.id
                        ? "Cancelando"
                        : "Cancelar"
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
                <span class="text-muted-foreground">Núcleo:</span>
                <span class="font-medium">{{ tramite.nucleo }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">Fecha de solicitud:</span>
                <span>{{ tramite.fecha_solicitud }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground"
                  >Fecha de finalización:</span
                >
                <span>{{ tramite.fecha_finalizado || "-" }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">Registrador:</span>
                <span>{{ tramite.registrador || "-" }}</span>
              </div>
            </div>
            <Button
              v-if="tramite.estado === 'Pendiente'"
              size="sm"
              variant="destructive"
              class="w-full gap-2"
              :disabled="loading_cancelar === tramite.id"
              @click="cancelarSolicitud(tramite.id)"
            >
              <Loader2
                v-if="loading_cancelar === tramite.id"
                class="size-4 animate-spin"
              />
              <XCircle v-else class="size-4" />
              {{ loading_cancelar === tramite.id ? "Cancelando" : "Cancelar" }}
            </Button>
          </div>
        </div>
        <div
          class="mt-6 flex flex-col gap-4 border-t pt-4 sm:flex-row sm:items-center sm:justify-between"
        >
          <div
            class="flex flex-wrap items-center gap-3 text-sm text-muted-foreground"
          >
            <span>Página {{ page }} de {{ totalPages }}</span>
            <span>({{ total }} trámites)</span>
          </div>
          <div class="flex flex-wrap items-center gap-4">
            <div class="flex items-center gap-2 text-sm">
              <span class="text-muted-foreground">Tamaño de página</span>
              <Select
                :model-value="String(limit)"
                @update:model-value="(value) => cambiarLimite(Number(value))"
              >
                <SelectTrigger class="h-9 w-[120px]">
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
              :page="page"
              :total="total"
              :sibling-count="1"
              :items-per-page="limit"
              @update:page="irAPagina"
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
                    :is-active="item.value === page"
                  >
                    {{ item.value }}
                  </PaginationItem>
                  <PaginationEllipsis v-else-if="item.type === 'ellipsis'" />
                </template>
                <PaginationNext />
                <PaginationLast />
              </PaginationContent>
            </Pagination>
          </div>
        </div>
      </template>
    </CardContent>
  </Card>
</template>
