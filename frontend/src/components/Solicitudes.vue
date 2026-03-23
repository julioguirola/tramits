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
    ClipboardList,
    Loader2,
    PlayCircle,
  },
  computed: {
    ...mapState(useTramiteStore, ["tramites", "cargando"]),
    tieneSolicitudes(): boolean {
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
    procesarSolicitud(id: string) {
      console.log("Procesar solicitud:", id);
      // TODO: Implementar lógica de procesamiento
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

        <!-- Sin solicitudes -->
        <div v-else-if="!tieneSolicitudes" class="py-8 text-center">
          <ClipboardList
            class="mx-auto size-12 text-muted-foreground opacity-50"
          />
          <p class="mt-4 text-sm text-muted-foreground">
            No hay solicitudes pendientes en este momento.
          </p>
        </div>

        <!-- Con solicitudes -->
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
                      class="gap-2"
                    >
                      <PlayCircle class="size-4" />
                      Procesar
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
              >
                <PlayCircle class="size-4" />
                Procesar
              </Button>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
