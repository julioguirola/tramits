<script lang="ts">
import { api } from "@/lib/utils";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  BarChart3,
  Package,
  Users,
  Building2,
  ClipboardList,
  CheckCircle,
  XCircle,
  Clock,
  Loader2,
} from "lucide-vue-next";

interface TramiteTipoCount {
  nombre: string;
  count: number;
}

interface TramiteMesCount {
  mes: string;
  count: number;
}

interface Estadisticas {
  total_tramites: number;
  pendientes: number;
  en_proceso: number;
  completados: number;
  rechazados: number;
  cancelados: number;
  total_bodegas: number;
  total_nucleos: number;
  total_personas_atendidas: number;
  tramites_por_tipo: TramiteTipoCount[];
  tramites_por_mes: TramiteMesCount[];
}

export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    Badge,
    Button,
    BarChart3,
    Package,
    Users,
    Building2,
    ClipboardList,
    CheckCircle,
    XCircle,
    Clock,
    Loader2,
  },
  data(): {
    estadisticas: Estadisticas | null;
    cargando: boolean;
  } {
    return {
      estadisticas: null,
      cargando: false,
    };
  },
  computed: {
    tieneEstadisticas(): boolean {
      return this.estadisticas !== null;
    },
    stats(): Estadisticas {
      return this.estadisticas as Estadisticas;
    },
    maxTipoCount(): number {
      const arr = this.stats.tramites_por_tipo;
      if (!arr?.length) return 1;
      return Math.max(...arr.map((t: TramiteTipoCount) => t.count));
    },
    maxMesCount(): number {
      const arr = this.stats.tramites_por_mes;
      if (!arr?.length) return 1;
      return Math.max(...arr.map((m: TramiteMesCount) => m.count));
    },
  },
  methods: {
    async cargarEstadisticas() {
      this.cargando = true;
      try {
        const res = await api.get("/tramite/estadisticas");
        if (res?.status === 200 && res.data?.data) {
          this.estadisticas = res.data.data;
        }
      } finally {
        this.cargando = false;
      }
    },
    getPorcentaje(value: number): number {
      if (!this.stats.total_tramites) return 0;
      return Math.round((value / this.stats.total_tramites) * 100);
    },
  },
  async mounted() {
    await this.cargarEstadisticas();
  },
};
</script>

<template>
  <div class="p-6 w-full space-y-6">
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2">
          <BarChart3 class="size-5" />
          Estadísticas de la Oficina
        </CardTitle>
        <CardDescription>
          Resumen de trámites, almacenes y atención a personas.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div v-if="cargando" class="flex items-center justify-center py-8">
          <Loader2 class="size-6 animate-spin text-muted-foreground" />
          <span class="ml-2 text-sm text-muted-foreground"
            >Cargando estadísticas...</span
          >
        </div>

        <template v-else-if="tieneEstadisticas">
          <!-- Tarjetas de resumen -->
          <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
            <div class="rounded-lg border p-4">
              <div
                class="flex items-center gap-2 text-sm font-medium text-muted-foreground mb-2"
              >
                <ClipboardList class="size-4" />
                Total Trámites
              </div>
              <div class="text-2xl font-bold">{{ stats.total_tramites }}</div>
            </div>
            <div class="rounded-lg border p-4">
              <div
                class="flex items-center gap-2 text-sm font-medium text-muted-foreground mb-2"
              >
                <Building2 class="size-4" />
                Bodegas
              </div>
              <div class="text-2xl font-bold">{{ stats.total_bodegas }}</div>
            </div>
            <div class="rounded-lg border p-4">
              <div
                class="flex items-center gap-2 text-sm font-medium text-muted-foreground mb-2"
              >
                <Package class="size-4" />
                Núcleos
              </div>
              <div class="text-2xl font-bold">{{ stats.total_nucleos }}</div>
            </div>
            <div class="rounded-lg border p-4">
              <div
                class="flex items-center gap-2 text-sm font-medium text-muted-foreground mb-2"
              >
                <Users class="size-4" />
                Personas Atendidas
              </div>
              <div class="text-2xl font-bold">
                {{ stats.total_personas_atendidas }}
              </div>
            </div>
          </div>

          <!-- Estado de trámites -->
          <div class="mt-6">
            <h3 class="text-lg font-semibold mb-4">Estado de Trámites</h3>
            <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-5">
              <div class="rounded-lg border p-4 bg-secondary/30">
                <div
                  class="flex items-center gap-2 text-sm font-medium text-muted-foreground mb-1"
                >
                  <Clock class="size-4 text-secondary-foreground" />
                  Pendientes
                </div>
                <div class="text-2xl font-bold">{{ stats.pendientes }}</div>
                <div class="text-xs text-muted-foreground mt-1">
                  {{ getPorcentaje(stats.pendientes) }}% del total
                </div>
              </div>
              <div
                class="rounded-lg border p-4 bg-yellow-50 dark:bg-yellow-900/20"
              >
                <div
                  class="flex items-center gap-2 text-sm font-medium text-yellow-700 dark:text-yellow-400 mb-1"
                >
                  <Clock class="size-4" />
                  En Proceso
                </div>
                <div class="text-2xl font-bold">{{ stats.en_proceso }}</div>
                <div class="text-xs text-muted-foreground mt-1">
                  {{ getPorcentaje(stats.en_proceso) }}% del total
                </div>
              </div>
              <div
                class="rounded-lg border p-4 bg-green-50 dark:bg-green-900/20"
              >
                <div
                  class="flex items-center gap-2 text-sm font-medium text-green-700 dark:text-green-400 mb-1"
                >
                  <CheckCircle class="size-4" />
                  Completados
                </div>
                <div class="text-2xl font-bold">{{ stats.completados }}</div>
                <div class="text-xs text-muted-foreground mt-1">
                  {{ getPorcentaje(stats.completados) }}% del total
                </div>
              </div>
              <div class="rounded-lg border p-4 bg-red-50 dark:bg-red-900/20">
                <div
                  class="flex items-center gap-2 text-sm font-medium text-red-700 dark:text-red-400 mb-1"
                >
                  <XCircle class="size-4" />
                  Rechazados
                </div>
                <div class="text-2xl font-bold">{{ stats.rechazados }}</div>
                <div class="text-xs text-muted-foreground mt-1">
                  {{ getPorcentaje(stats.rechazados) }}% del total
                </div>
              </div>
              <div
                class="rounded-lg border p-4 bg-slate-50 dark:bg-slate-900/20"
              >
                <div
                  class="flex items-center gap-2 text-sm font-medium text-slate-700 dark:text-slate-400 mb-1"
                >
                  <XCircle class="size-4" />
                  Cancelados
                </div>
                <div class="text-2xl font-bold">{{ stats.cancelados }}</div>
                <div class="text-xs text-muted-foreground mt-1">
                  {{ getPorcentaje(stats.cancelados) }}% del total
                </div>
              </div>
            </div>
          </div>
        </template>

        <div v-else class="py-8 text-center">
          <BarChart3 class="mx-auto size-12 text-muted-foreground opacity-50" />
          <p class="mt-4 text-sm text-muted-foreground">
            No hay estadísticas disponibles.
          </p>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

