import { defineStore } from "pinia";
import { api } from "@/lib/utils";

export interface Tramite {
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

export const useTramiteStore = defineStore("tramite-store", {
  state: () => ({
    tramites: null as Tramite[] | null,
    tramites_en_proceso: null as Tramite[] | null,
    cargando: false,
  }),
  actions: {
    async cargarHistorial(estadoId?: number, asignadas?: boolean) {
      this.cargando = true;
      try {
        const params = new URLSearchParams();
        if (estadoId !== undefined) {
          params.append("estado_id", estadoId.toString());
        }
        if (asignadas !== undefined) {
          params.append("asignadas", String(asignadas));
        }
        const url = `/tramite${params.toString() ? `?${params.toString()}` : ""}`;
        const res = await api.get(url);
        if (res?.status === 200) {
          const data = res.data.data || [];
          if (estadoId === 2 && asignadas === true) {
            this.tramites_en_proceso = data;
          } else {
            this.tramites = data;
          }
        }
      } catch (error) {
        console.error("Error al cargar historial:", error);
      } finally {
        this.cargando = false;
      }
    },
    async procesarBaja(tramiteId: string): Promise<boolean> {
      try {
        const res = await api.post(`/tramite/${tramiteId}/procesar`);
        return res?.status === 200;
      } catch {
        return false;
      }
    },
    async gestionarTramite(
      tramiteId: string,
      accion: "completar" | "rechazar",
    ): Promise<boolean> {
      try {
        const res = await api.post(`/tramite/${tramiteId}/gestionar`, { accion });
        return res?.status === 200;
      } catch {
        return false;
      }
    },
    limpiar() {
      this.tramites = null;
      this.tramites_en_proceso = null;
      this.cargando = false;
    },
  },
});
