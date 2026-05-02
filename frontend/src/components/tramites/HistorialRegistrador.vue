<script lang="ts">
import { api } from "@/lib/utils";
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
import { ClipboardList, Loader2, Download } from "lucide-vue-next";
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

interface TramiteHistorial {
  id: string;
  tipo: string;
  nucleo: string;
  fecha_solicitud: string;
  fecha_finalizado: string | null;
  registrador: string | null;
  estado: string;
  persona_nombre: string | null;
  persona_apellido: string | null;
}

interface HistorialResponse {
  tramites: TramiteHistorial[];
  total: number;
  page: number;
  limit: number;
}

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
    ClipboardList,
    Loader2,
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
  data(): {
    tramites: TramiteHistorial[];
    cargando: boolean;
    page: number;
    limit: number;
    total: number;
    limitOptions: number[];
  } {
    return {
      tramites: [],
      cargando: false,
      page: 1,
      limit: 10,
      total: 0,
      limitOptions: [5, 10, 20, 30],
    };
  },
  computed: {
    tieneTramites(): boolean {
      return this.tramites.length > 0;
    },
    totalPages(): number {
      return Math.max(1, Math.ceil(this.total / this.limit));
    },
  },
  methods: {
    getEstadoVariant(
      estado: string,
    ): "default" | "secondary" | "outline" | "destructive" {
      if (estado === "Completado") return "default";
      if (estado === "Rechazado") return "destructive";
      if (estado === "Cancelado") return "outline";
      return "secondary";
    },
    getNombreCompleto(tramite: TramiteHistorial): string {
      if (tramite.persona_nombre && tramite.persona_apellido) {
        return `${tramite.persona_nombre} ${tramite.persona_apellido}`;
      }
      return "-";
    },
    async cargarHistorial() {
      this.cargando = true;
      try {
        const res = await api.get(
          `/tramite/historial/registrador?page=${this.page}&limit=${this.limit}`,
        );
        if (res?.status === 200 && res.data?.data) {
          const data: HistorialResponse = res.data.data;
          this.tramites = data.tramites || [];
          this.total = data.total || 0;
          this.page = data.page || this.page;
          this.limit = data.limit || this.limit;
        }
      } finally {
        this.cargando = false;
      }
    },
    async irAPagina(page: number) {
      if (this.page === page || this.cargando || page < 1) return;
      this.page = page;
      await this.cargarHistorial();
    },
    async cambiarLimite(limite: number) {
      if (this.cargando || this.limit === limite) return;
      this.limit = limite;
      this.page = 1;
      await this.cargarHistorial();
    },
    async siguientePagina() {
      await this.irAPagina(this.page + 1);
    },
    async paginaAnterior() {
      await this.irAPagina(this.page - 1);
    },
    exportarExcel() {
      if (!this.tramites || this.tramites.length === 0) return;
      const data = this.tramites.map((t) => ({
        Solicitante: this.getNombreCompleto(t),
        Tipo: t.tipo,
        Núcleo: t.nucleo,
        "Fecha de Solicitud": t.fecha_solicitud,
        "Fecha de Finalización": t.fecha_finalizado || "-",
        Estado: t.estado,
      }));
      const ws = XLSX.utils.json_to_sheet(data);
      const wb = XLSX.utils.book_new();
      XLSX.utils.book_append_sheet(wb, ws, "Historial");
      XLSX.writeFile(
        wb,
        `historial_registrador_${new Date().toISOString().split("T")[0]}.xlsx`,
      );
    },
  },
  async mounted() {
    await this.cargarHistorial();
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
            Historial
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
          Revisa tus trámites finalizados (completados o rechazados).
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div v-if="cargando" class="flex items-center justify-center py-8">
          <Loader2 class="size-6 animate-spin text-muted-foreground" />
          <span class="ml-2 text-sm text-muted-foreground"
            >Cargando historial...</span
          >
        </div>

        <div v-else-if="!tieneTramites" class="py-8 text-center">
          <ClipboardList
            class="mx-auto size-12 text-muted-foreground opacity-50"
          />
          <p class="mt-4 text-sm text-muted-foreground">
            No tienes trámites finalizados aún.
          </p>
        </div>

        <template v-else>
          <div class="hidden rounded-md border md:block">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Solicitante</TableHead>
                  <TableHead>Tipo</TableHead>
                  <TableHead>Núcleo</TableHead>
                  <TableHead>Fecha de solicitud</TableHead>
                  <TableHead>Fecha de finalización</TableHead>
                  <TableHead>Estado</TableHead>
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
                  <TableCell>{{ tramite.fecha_finalizado || "-" }}</TableCell>
                  <TableCell>
                    <Badge :variant="getEstadoVariant(tramite.estado)">
                      {{ tramite.estado }}
                    </Badge>
                  </TableCell>
                </TableRow>
              </TableBody>
            </Table>
          </div>

          <div class="space-y-4 md:hidden">
            <div
              v-for="tramite in tramites"
              :key="tramite.id"
              class="rounded-lg border p-4 space-y-3"
            >
              <div class="flex items-center justify-between">
                <span class="font-semibold text-sm">{{ tramite.tipo }}</span>
                <Badge :variant="getEstadoVariant(tramite.estado)">
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
                  <span class="text-muted-foreground">Fecha de solicitud:</span>
                  <span>{{ tramite.fecha_solicitud }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-muted-foreground"
                    >Fecha de finalización:</span
                  >
                  <span>{{ tramite.fecha_finalizado || "-" }}</span>
                </div>
              </div>
            </div>
          </div>
        </template>

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
      </CardContent>
    </Card>
  </div>
</template>
