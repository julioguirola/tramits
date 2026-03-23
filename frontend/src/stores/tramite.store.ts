import { defineStore } from "pinia";
import { api } from "@/lib/utils";

export interface Tramite {
  id: string;
  tipo: string;
  nucleo: string;
  fecha_solicitud: string;
  fecha_completado: string | null;
  registrador: string | null;
  estado: string;
  persona_nombre: string | null;
  persona_apellido: string | null;
}

export const useTramiteStore = defineStore("tramite-store", {
  state: () => ({
    tramites: null as Tramite[] | null,
    cargando: false,
  }),
  actions: {
    async cargarHistorial(estadoId?: number) {
      this.cargando = true;
      try {
        const params = new URLSearchParams();
        if (estadoId !== undefined) {
          params.append("estado_id", estadoId.toString());
        }
        const url = `/tramite${params.toString() ? `?${params.toString()}` : ""}`;
        const res = await api.get(url);
        if (res?.status === 200) {
          this.tramites = res.data.data || [];
        }
      } catch (error) {
        console.error("Error al cargar historial:", error);
      } finally {
        this.cargando = false;
      }
    },
    limpiar() {
      this.tramites = null;
      this.cargando = false;
    },
  },
});
