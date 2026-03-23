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

      <!-- Tabla con trámites -->
      <div v-else class="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Tipo</TableHead>
              <TableHead>Núcleo</TableHead>
              <TableHead>Fecha Solicitud</TableHead>
              <TableHead>Registrador</TableHead>
              <TableHead>Fecha Completado</TableHead>
              <TableHead>Estado</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-for="tramite in tramites" :key="tramite.id">
              <TableCell class="font-medium">{{ tramite.tipo }}</TableCell>
              <TableCell>{{ tramite.nucleo }}</TableCell>
              <TableCell>{{ tramite.fecha_solicitud }}</TableCell>
              <TableCell>
                {{ tramite.registrador || "-" }}
              </TableCell>
              <TableCell>
                {{ tramite.fecha_completado || "-" }}
              </TableCell>
              <TableCell>
                <Badge
                  :variant="getEstadoVariant(tramite.estado)"
                  :class="
                    tramite.estado === 'En proceso' ? 'bg-yellow-500' : ''
                  "
                >
                  {{ tramite.estado }}
                </Badge>
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </div>
    </CardContent>
  </Card>
</template>
