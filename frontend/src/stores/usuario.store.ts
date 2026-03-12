import { defineStore } from "pinia";
import { api } from "@/lib/utils";

export interface Usuario {
  nombre: string;
  apellido: string;
  email: string;
  rol: string;
  avatar: string;
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
