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
import { ClipboardList, Loader2 } from "lucide-vue-next";

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
  },
  data(): {
    tramites: TramiteHistorial[];
    cargando: boolean;
    page: number;
    limit: number;
    total: number;
  } {
    return {
      tramites: [],
      cargando: false,
      page: 1,
      limit: 10,
      total: 0,
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
    async siguientePagina() {
      if (this.page >= this.totalPages || this.cargando) return;
      this.page += 1;
      await this.cargarHistorial();
    },
    async paginaAnterior() {
      if (this.page <= 1 || this.cargando) return;
      this.page -= 1;
      await this.cargarHistorial();
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
        <CardTitle class="flex items-center gap-2">
          <ClipboardList class="size-5" />
          Historial
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
                  <TableHead>Fecha Solicitud</TableHead>
                  <TableHead>Fecha Finalizado</TableHead>
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
                  <span class="font-medium">{{ getNombreCompleto(tramite) }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-muted-foreground">Núcleo:</span>
                  <span>{{ tramite.nucleo }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-muted-foreground">Fecha Solicitud:</span>
                  <span>{{ tramite.fecha_solicitud }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-muted-foreground">Fecha Finalizado:</span>
                  <span>{{ tramite.fecha_finalizado || "-" }}</span>
                </div>
              </div>
            </div>
          </div>
        </template>

        <div class="mt-6 flex items-center justify-between border-t pt-4">
          <p class="text-sm text-muted-foreground">
            Página {{ page }} de {{ totalPages }} ({{ total }} trámites)
          </p>
          <div class="flex items-center gap-2">
            <Button
              variant="outline"
              size="sm"
              :disabled="page <= 1 || cargando"
              @click="paginaAnterior"
            >
              Anterior
            </Button>
            <Button
              variant="outline"
              size="sm"
              :disabled="page >= totalPages || cargando"
              @click="siguientePagina"
            >
              Siguiente
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
