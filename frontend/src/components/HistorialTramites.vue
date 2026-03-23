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
import { ClipboardList, Loader2 } from "lucide-vue-next";

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
  },
  computed: {
    ...mapState(useTramiteStore, ["tramites", "cargando"]),
    tieneTramites(): boolean {
      return Boolean(this.tramites && this.tramites.length > 0);
    },
  },
  methods: {
    ...mapActions(useTramiteStore, ["cargarHistorial"]),
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
  },
  async mounted() {
    await this.cargarHistorial();
  },
};
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle class="flex items-center gap-2">
        <ClipboardList class="size-5" />
        Historial de Trámites
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
                <TableHead>Fecha Solicitud</TableHead>
                <TableHead>Fecha Completado</TableHead>
                <TableHead>Registrador</TableHead>
                <TableHead>Estado</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow v-for="tramite in tramites" :key="tramite.id">
                <TableCell class="font-medium">{{ tramite.tipo }}</TableCell>
                <TableCell>{{ tramite.nucleo }}</TableCell>
                <TableCell>{{ tramite.fecha_solicitud }}</TableCell>
                <TableCell>
                  {{ tramite.fecha_completado || "-" }}
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
                <span class="text-muted-foreground">Fecha Solicitud:</span>
                <span>{{ tramite.fecha_solicitud }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">Fecha Completado:</span>
                <span>{{ tramite.fecha_completado || "-" }}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">Registrador:</span>
                <span>{{ tramite.registrador || "-" }}</span>
              </div>
            </div>
          </div>
        </div>
      </template>
    </CardContent>
  </Card>
</template>
