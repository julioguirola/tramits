import { defineStore } from "pinia";
import { api } from "@/lib/utils";

export interface Usuario {
  nombre: string;
  apellido: string;
  email: string;
  rol: string;
  avatar: string;
  nucleo: string | null;
  bodega: string | null;
  oficina: string | null;
  municipio: string | null;
  provincia: string | null;
  numero_nucleo: string | null;
}

export const useUsuarioStore = defineStore("usuario", {
  state: () => ({
    usuario: null as Usuario | null,
    cargando: false,
  }),
  actions: {
    async cargar() {
      if (this.usuario) return;
      this.cargando = true;
      try {
        const res = await api.get("/usuario/me");
        if (res?.status === 200 && res.data?.data) {
          const d = res.data.data;
          this.usuario = {
            nombre: d.nombre,
            apellido: d.apellido,
            email: d.email,
            rol: d.rol,
            avatar: "",
            nucleo: d.nucleo ?? null,
            numero_nucleo: d.numero_nucleo ?? null,
            bodega: d.bodega ?? null,
            oficina: d.oficina ?? null,
            municipio: d.municipio ?? null,
            provincia: d.provincia ?? null,
          };
        }
      } finally {
        this.cargando = false;
      }
    },
    limpiar() {
      this.usuario = null;
    },
  },
});
