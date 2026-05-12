<script lang="ts">
import { api } from "@/lib/utils";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

interface NomItem {
  id: number;
  nombre: string;
}

export default {
  components: {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectTrigger,
    SelectValue,
  },
  emits: ["update:oficinaId"],
  props: {
    oficinaId: { type: Number, default: null },
  },
  data() {
    return {
      provincias: [] as NomItem[],
      provinciaId: null as number | null,
      municipios: [] as NomItem[],
      municipioId: null as number | null,
      oficinas: [] as NomItem[],
      oficinaIdLocal: null as number | null,
    };
  },
  watch: {
    provinciaId(val) {
      this.municipioId = null;
      this.oficinaIdLocal = null;
      this.municipios = [];
      this.oficinas = [];
      if (val) this.cargarMunicipios(val);
    },
    municipioId(val) {
      this.oficinaIdLocal = null;
      this.oficinas = [];
      if (val) this.cargarOficinas(val);
    },
    oficinaIdLocal(val) {
      this.$emit("update:oficinaId", val);
    },
  },
  methods: {
    async cargarProvincias() {
      try {
        const res = await api.get("/provincia");
        if (res?.status === 200 && res.data?.data) {
          this.provincias = res.data.data;
        }
      } catch {}
    },
    async cargarMunicipios(provinciaId: number) {
      try {
        const res = await api.get("/municipio", { params: { provincia_id: provinciaId } });
        if (res?.status === 200 && res.data?.data) {
          this.municipios = res.data.data;
        }
      } catch {}
    },
    async cargarOficinas(municipioId: number) {
      try {
        const res = await api.get("/oficina", {
          params: { municipio_id: municipioId, page: 1, limit: 200 },
        });
        if (res?.status === 200 && res.data?.data) {
          this.oficinas = res.data.data;
        }
      } catch {}
    },
  },
  async mounted() {
    await this.cargarProvincias();
  },
};
</script>

<template>
  <div class="flex flex-wrap gap-3">
    <Select v-model="provinciaId">
      <SelectTrigger class="w-45">
        <SelectValue placeholder="Provincia" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectLabel>Provincias</SelectLabel>
          <SelectItem v-for="p in provincias" :key="p.id" :value="p.id">
            {{ p.nombre }}
          </SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>
    <Select v-model="municipioId" :disabled="!provinciaId">
      <SelectTrigger class="w-45">
        <SelectValue placeholder="Municipio" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectLabel>Municipios</SelectLabel>
          <SelectItem v-for="m in municipios" :key="m.id" :value="m.id">
            {{ m.nombre }}
          </SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>
    <Select v-model="oficinaIdLocal" :disabled="!municipioId">
      <SelectTrigger class="w-45">
        <SelectValue placeholder="Oficina" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectLabel>Oficinas</SelectLabel>
          <SelectItem v-for="o in oficinas" :key="o.id" :value="o.id">
            {{ o.nombre }}
          </SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>
  </div>
</template>
