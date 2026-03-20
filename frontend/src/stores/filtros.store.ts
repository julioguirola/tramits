import { api } from "@/lib/utils";
import { defineStore } from "pinia";
interface NomFiltro {
  id: number;
  nombre: string;
}

export const filtrosStore = defineStore("filtros", {
  state: () => ({
    bodega_id: null as number | null,
    oficina_id: null as number | null,
    municipio_id: null as number | null,
    provincia_id: null as number | null,
    provincias: [] as NomFiltro[],
    municipios: [] as NomFiltro[],
    oficinas: [] as NomFiltro[],
    bodegas: [] as NomFiltro[],
  }),
  actions: {
    setBodega(id: number) {
      this.bodega_id = id;
    },
    async setOficina(id: number) {
      this.oficina_id = id;
      this.bodega_id = null;
      this.bodegas = [];
      await this.getBodegas();
    },
    async setMunicipio(id: number) {
      this.municipio_id = id;
      this.oficina_id = null;
      this.bodega_id = null;
      this.oficinas = [];
      this.bodegas = [];
      await this.getOficinas();
    },
    async setProvincia(id: number) {
      this.provincia_id = id;
      this.oficina_id = null;
      this.bodega_id = null;
      this.municipio_id = null;
      this.oficinas = [];
      this.bodegas = [];
      this.municipios = [];
      await this.getMunicipios();
    },
    async getProvincias() {
      this.provincias = (await api.get("/provincia")).data.data;
    },
    async getMunicipios() {
      this.municipios = (
        await api.get("/municipio", {
          params: {
            provincia_id: this.provincia_id,
          },
        })
      ).data.data;
    },
    async getOficinas() {
      this.oficinas = (
        await api.get("/oficina", {
          params: {
            municipio_id: this.municipio_id,
          },
        })
      ).data.data;
    },
    async getBodegas() {
      this.bodegas = (
        await api.get("/bodega", {
          params: {
            oficina_id: this.oficina_id,
          },
        })
      ).data.data;
    },
  },
});
