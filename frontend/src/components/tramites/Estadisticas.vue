<script lang="ts">
import { defineAsyncComponent } from "vue";
import { api } from "@/lib/utils";
import { exportToPdf } from "@/composables/useExportPdf";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import {
  Loader2,
  Files,
  Clock,
  CheckCircle,
  XCircle,
  Warehouse,
  Hexagon,
  Users,
  Loader,
  FileDown,
} from "lucide-vue-next";
import type { ChartConfig } from "@/components/ui/chart";
import { filtrosStore } from "@/stores/filtros.store";
import { useUsuarioStore } from "@/stores/usuario.store";
import { mapState } from "pinia";
import ProvinciaFiltro from "@/components/filtros/ProvinciaFiltro.vue";
import MunicipioFiltro from "@/components/filtros/MunicipioFiltro.vue";
import OficinaFiltro from "@/components/filtros/OficinaFiltro.vue";

const GraficaPastel = defineAsyncComponent(() => import("./GraficaPastel.vue"));
const GraficaBarras = defineAsyncComponent(() => import("./GraficaBarras.vue"));

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
  tramites_por_tipo: { nombre: string; count: number }[];
  tramites_por_mes: { mes: string; count: number }[];
}

export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardTitle,
    CardHeader,
    Button,
    Loader2,
    Files,
    Clock,
    Loader,
    CheckCircle,
    XCircle,
    Warehouse,
    Hexagon,
    Users,
    GraficaPastel,
    GraficaBarras,
    FileDown,
    ProvinciaFiltro,
    MunicipioFiltro,
    OficinaFiltro,
  },
  data() {
    return {
      estadisticas: null as Estadisticas | null,
      cargando: false,
      exportando: false,
      error: null as string | null,
      filtrosKey: 0,
      chartConfig: {
        tramites: {
          label: "Trámites",
          color: "var(--chart-1)",
        },
      } satisfies ChartConfig,
    };
  },
  computed: {
    ...mapState(filtrosStore, ["provincia_id", "municipio_id", "oficina_id"]),
    ...mapState(useUsuarioStore, ["usuario"]),
    esAdmin(): boolean {
      return this.usuario?.rol === "Administrador";
    },
    hayFiltrosActivos(): boolean {
      return Boolean(this.provincia_id || this.municipio_id || this.oficina_id);
    },
    estadoData() {
      if (!this.estadisticas) return [];
      return [
        {
          estado: "Pendientes",
          cantidad: this.estadisticas.pendientes,
          color: "#f59e0b",
        },
        {
          estado: "En Proceso",
          cantidad: this.estadisticas.en_proceso,
          color: "#3b82f6",
        },
        {
          estado: "Completados",
          cantidad: this.estadisticas.completados,
          color: "#22c55e",
        },
        {
          estado: "Rechazados",
          cantidad: this.estadisticas.rechazados,
          color: "#ef4444",
        },
        {
          estado: "Cancelados",
          cantidad: this.estadisticas.cancelados,
          color: "#6b7280",
        },
      ];
    },
    tiposData() {
      if (!this.estadisticas?.tramites_por_tipo) return [];
      return [...this.estadisticas.tramites_por_tipo].sort(
        (a, b) => b.count - a.count,
      );
    },
    chartData() {
      if (!this.estadisticas) return [];
      return this.estadisticas.tramites_por_mes.map((item, index) => ({
        ...item,
        index,
      }));
    },
  },
  methods: {
    async cargarEstadisticas() {
      this.cargando = true;
      this.error = null;
      try {
        const res = await api.get("/tramite/estadisticas", {
          params: {
            provincia_id: this.esAdmin ? this.provincia_id ?? undefined : undefined,
            municipio_id: this.esAdmin ? this.municipio_id ?? undefined : undefined,
            oficina_id: this.esAdmin ? this.oficina_id ?? undefined : undefined,
          },
        });
        if (res?.status === 200 && res.data?.data) {
          this.estadisticas = res.data.data;
        } else {
          this.error = "Error al cargar las estadísticas";
        }
      } catch (err: any) {
        this.error =
          err.response?.data?.description || "Error al cargar las estadísticas";
        console.error("Error al cargar estadísticas:", err);
      } finally {
        this.cargando = false;
      }
    },
    async exportarPdf() {
      this.exportando = true;
      try {
        await exportToPdf("estadisticas-export", "estadisticas");
      } catch (err) {
        console.error("Error al exportar PDF:", err);
      } finally {
        this.exportando = false;
      }
    },
    async limpiarFiltros() {
      const filtros = filtrosStore();
      filtros.$patch({
        provincia_id: null,
        municipio_id: null,
        oficina_id: null,
        municipios: [],
        oficinas: [],
      });
      this.filtrosKey += 1;
      await this.cargarEstadisticas();
    },
  },
  watch: {
    provincia_id() {
      if (!this.esAdmin) return;
      this.cargarEstadisticas();
    },
    municipio_id() {
      if (!this.esAdmin) return;
      this.cargarEstadisticas();
    },
    oficina_id() {
      if (!this.esAdmin) return;
      this.cargarEstadisticas();
    },
    esAdmin(nuevoValor, valorAnterior) {
      if (nuevoValor !== valorAnterior) {
        this.cargarEstadisticas();
      }
    },
  },
  async mounted() {
    await this.cargarEstadisticas();
  },
};
</script>

<template>
  <div class="p-6 w-full space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold tracking-tight">Estadísticas</h2>
        <p class="text-muted-foreground">
          Visualización de datos y métricas de trámites
        </p>
      </div>
      <div class="flex flex-wrap items-center gap-3">
        <Button
          variant="default"
          size="sm"
          class="gap-2"
          @click="exportarPdf"
          :disabled="exportando || cargando"
        >
          <FileDown v-if="!exportando" class="size-4" />
          <Loader2 v-else class="size-4 animate-spin" />
          Exportar PDF
        </Button>
      </div>
    </div>

    <div
      v-if="esAdmin"
      class="flex flex-wrap items-center gap-3"
      :key="filtrosKey"
    >
      <ProvinciaFiltro />
      <MunicipioFiltro />
      <OficinaFiltro />
      <Button
        v-if="hayFiltrosActivos"
        variant="outline"
        size="sm"
        @click="limpiarFiltros"
      >
        Limpiar filtros
      </Button>
    </div>

    <div
      v-if="cargando && !estadisticas"
      class="flex items-center justify-center py-20"
    >
      <Loader2 class="size-8 animate-spin text-muted-foreground" />
      <span class="ml-2 text-muted-foreground">Cargando estadísticas...</span>
    </div>

    <div
      v-else-if="error"
      class="rounded-lg border border-destructive/50 bg-destructive/10 p-6 text-center"
    >
      <p class="text-destructive">{{ error }}</p>
      <Button variant="outline" class="mt-4" @click="cargarEstadisticas">
        Reintentar
      </Button>
    </div>

    <template v-else-if="estadisticas">
      <div id="estadisticas-export" class="space-y-6">
        <div class="grid gap-4 md:grid-cols-3 lg:grid-cols-5">
          <Card class="border-l-4 border-l-primary">
            <CardHeader
              class="pb-2 flex flex-row items-center justify-between space-y-0"
            >
              <CardDescription>Total Trámites</CardDescription>
              <Files class="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <CardTitle class="text-3xl">{{
                estadisticas.total_tramites
              }}</CardTitle>
            </CardContent>
          </Card>
          <Card class="border-l-4 border-l-yellow-500">
            <CardHeader
              class="pb-2 flex flex-row items-center justify-between space-y-0"
            >
              <CardDescription>Pendientes</CardDescription>
              <Clock class="h-4 w-4 text-yellow-500" />
            </CardHeader>
            <CardContent>
              <CardTitle class="text-3xl text-yellow-600">{{
                estadisticas.pendientes
              }}</CardTitle>
            </CardContent>
          </Card>
          <Card class="border-l-4 border-l-blue-500">
            <CardHeader
              class="pb-2 flex flex-row items-center justify-between space-y-0"
            >
              <CardDescription>En Proceso</CardDescription>
              <Loader class="h-4 w-4 text-blue-500" />
            </CardHeader>
            <CardContent>
              <CardTitle class="text-3xl text-blue-600">{{
                estadisticas.en_proceso
              }}</CardTitle>
            </CardContent>
          </Card>
          <Card class="border-l-4 border-l-green-500">
            <CardHeader
              class="pb-2 flex flex-row items-center justify-between space-y-0"
            >
              <CardDescription>Completados</CardDescription>
              <CheckCircle class="h-4 w-4 text-green-500" />
            </CardHeader>
            <CardContent>
              <CardTitle class="text-3xl text-green-600">{{
                estadisticas.completados
              }}</CardTitle>
            </CardContent>
          </Card>
          <Card class="border-l-4 border-l-red-500">
            <CardHeader
              class="pb-2 flex flex-row items-center justify-between space-y-0"
            >
              <CardDescription>Rechazados</CardDescription>
              <XCircle class="h-4 w-4 text-red-500" />
            </CardHeader>
            <CardContent>
              <CardTitle class="text-3xl text-red-600">{{
                estadisticas.rechazados
              }}</CardTitle>
            </CardContent>
          </Card>
        </div>

        <div class="grid gap-4 md:grid-cols-3">
          <Card class="border-l-4 border-l-purple-500">
            <CardHeader
              class="pb-2 flex flex-row items-center justify-between space-y-0"
            >
              <CardDescription>Bodegas</CardDescription>
              <Warehouse class="h-4 w-4 text-purple-500" />
            </CardHeader>
            <CardContent>
              <CardTitle class="text-3xl">{{
                estadisticas.total_bodegas
              }}</CardTitle>
            </CardContent>
          </Card>
          <Card class="border-l-4 border-l-cyan-500">
            <CardHeader
              class="pb-2 flex flex-row items-center justify-between space-y-0"
            >
              <CardDescription>Núcleos</CardDescription>
              <Hexagon class="h-4 w-4 text-cyan-500" />
            </CardHeader>
            <CardContent>
              <CardTitle class="text-3xl">{{
                estadisticas.total_nucleos
              }}</CardTitle>
            </CardContent>
          </Card>
          <Card class="border-l-4 border-l-orange-500">
            <CardHeader
              class="pb-2 flex flex-row items-center justify-between space-y-0"
            >
              <CardDescription>Personas Atendidas</CardDescription>
              <Users class="h-4 w-4 text-orange-500" />
            </CardHeader>
            <CardContent>
              <CardTitle class="text-3xl">{{
                estadisticas.total_personas_atendidas
              }}</CardTitle>
            </CardContent>
          </Card>
        </div>

        <div class="grid gap-6 lg:grid-cols-2">
          <Card>
            <CardHeader>
              <CardTitle>Trámites por Estado</CardTitle>
              <CardDescription>Distribución actual de trámites</CardDescription>
            </CardHeader>
            <CardContent>
              <div class="space-y-3">
                <div
                  v-for="item in estadoData"
                  :key="item.estado"
                  class="flex items-center gap-3"
                >
                  <div
                    class="w-3 h-3 rounded-full"
                    :style="{ backgroundColor: item.color }"
                  ></div>
                  <span class="flex-1 text-sm">{{ item.estado }}</span>
                  <span class="text-sm font-medium">{{ item.cantidad }}</span>
                  <div class="w-32 h-2 bg-muted rounded-full overflow-hidden">
                    <div
                      class="h-full rounded-full"
                      :style="{
                        backgroundColor: item.color,
                        width:
                          estadisticas.total_tramites > 0
                            ? `${(item.cantidad / estadisticas.total_tramites) * 100}%`
                            : '0%',
                      }"
                    ></div>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>

          <Suspense>
            <GraficaPastel
              :tipos-data="tiposData"
              :chart-config="chartConfig"
            />
            <template #fallback>
              <Card class="flex items-center justify-center h-72">
                <Loader2 class="size-6 animate-spin text-muted-foreground" />
              </Card>
            </template>
          </Suspense>
        </div>

        <Suspense>
          <GraficaBarras :chart-data="chartData" :chart-config="chartConfig" />
          <template #fallback>
            <Card class="flex items-center justify-center h-96">
              <Loader2 class="size-6 animate-spin text-muted-foreground" />
            </Card>
          </template>
        </Suspense>
      </div>
    </template>
  </div>
</template>
