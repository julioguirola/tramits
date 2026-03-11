import { defineStore } from "pinia";
import { ref } from "vue";
import { api } from "@/lib/utils";

export interface Usuario {
  nombre: string;
  apellido: string;
  email: string;
  tipo: string;
  avatar: string;
}

export const useUsuarioStore = defineStore("usuario", () => {
  const usuario = ref<Usuario | null>(null);
  const cargando = ref(false);

  async function cargar() {
    if (usuario.value) return;
    cargando.value = true;
    try {
      const res = await api.get("/usuario/me");
      if (res?.status === 200 && res.data?.data) {
        const d = res.data.data;
        usuario.value = {
          nombre: d.nombre,
          apellido: d.apellido,
          email: d.email,
          tipo: d.tipo,
          avatar: "",
        };
      }
    } finally {
      cargando.value = false;
    }
  }

  function limpiar() {
    usuario.value = null;
  }

  return { usuario, cargando, cargar, limpiar };
});
