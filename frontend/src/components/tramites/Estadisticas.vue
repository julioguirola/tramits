<script lang="ts">
import { api } from "@/lib/utils";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Loader2, RefreshCw } from "lucide-vue-next";

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
    RefreshCw,
  },
  data() {
    return {
      estadisticas: null as Estadisticas | null,
      cargando: false,
      error: null as string | null,
    };
  },
  computed: {
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
  },
  methods: {
    async cargarEstadisticas() {
      this.cargando = true;
      this.error = null;
      try {
        const res = await api.get("/tramite/estadisticas");
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
      <Button
        variant="outline"
        size="sm"
        class="gap-2"
        @click="cargarEstadisticas"
        :disabled="cargando"
      >
        <RefreshCw v-if="!cargando" class="size-4" />
        <Loader2 v-else class="size-4 animate-spin" />
        Actualizar
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
      <div class="grid gap-4 md:grid-cols-3 lg:grid-cols-5">
        <Card>
          <CardHeader class="pb-2">
            <CardDescription>Total Trámites</CardDescription>
            <CardTitle class="text-3xl">{{
              estadisticas.total_tramites
            }}</CardTitle>
          </CardHeader>
        </Card>
        <Card>
          <CardHeader class="pb-2">
            <CardDescription>Pendientes</CardDescription>
            <CardTitle class="text-3xl text-yellow-600">{{
              estadisticas.pendientes
            }}</CardTitle>
          </CardHeader>
        </Card>
        <Card>
          <CardHeader class="pb-2">
            <CardDescription>En Proceso</CardDescription>
            <CardTitle class="text-3xl text-blue-600">{{
              estadisticas.en_proceso
            }}</CardTitle>
          </CardHeader>
        </Card>
        <Card>
          <CardHeader class="pb-2">
            <CardDescription>Completados</CardDescription>
            <CardTitle class="text-3xl text-green-600">{{
              estadisticas.completados
            }}</CardTitle>
          </CardHeader>
        </Card>
        <Card>
          <CardHeader class="pb-2">
            <CardDescription>Rechazados</CardDescription>
            <CardTitle class="text-3xl text-red-600">{{
              estadisticas.rechazados
            }}</CardTitle>
          </CardHeader>
        </Card>
      </div>

      <div class="grid gap-4 md:grid-cols-3">
        <Card>
          <CardHeader class="pb-2">
            <CardDescription>Bodegas</CardDescription>
            <CardTitle class="text-3xl">{{
              estadisticas.total_bodegas
            }}</CardTitle>
          </CardHeader>
        </Card>
        <Card>
          <CardHeader class="pb-2">
            <CardDescription>Núcleos</CardDescription>
            <CardTitle class="text-3xl">{{
              estadisticas.total_nucleos
            }}</CardTitle>
          </CardHeader>
        </Card>
        <Card>
          <CardHeader class="pb-2">
            <CardDescription>Personas Atendidas</CardDescription>
            <CardTitle class="text-3xl">{{
              estadisticas.total_personas_atendidas
            }}</CardTitle>
          </CardHeader>
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

        <Card>
          <CardHeader>
            <CardTitle>Top Tipos de Trámite</CardTitle>
            <CardDescription>Los 10 tipos más solicitados</CardDescription>
          </CardHeader>
          <CardContent>
            <div class="space-y-3">
              <div
                v-for="(tipo, index) in tiposData.slice(0, 10)"
                :key="tipo.nombre"
                class="flex items-center gap-3"
              >
                <span class="text-xs text-muted-foreground w-6">{{
                  index + 1
                }}</span>
                <span class="flex-1 text-sm truncate">{{ tipo.nombre }}</span>
                <span class="text-sm font-medium">{{ tipo.count }}</span>
                <div class="w-32 h-2 bg-muted rounded-full overflow-hidden">
                  <div
                    class="h-full rounded-full bg-primary"
                    :style="{
                      width:
                        tiposData.length > 0 && tiposData[0]
                          ? `${(tipo.count / tiposData[0].count) * 100}%`
                          : '0%',
                    }"
                  ></div>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </template>
  </div>
</template>
