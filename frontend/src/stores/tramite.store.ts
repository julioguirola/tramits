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
}

export const useTramiteStore = defineStore("tramite-store", {
  state: () => ({
    tramites: null as Tramite[] | null,
    cargando: false,
  }),
  actions: {
    async cargarHistorial() {
      this.cargando = true;
      try {
        const res = await api.get("/tramite/historial");
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
