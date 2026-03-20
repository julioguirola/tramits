<script lang="ts">
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { api } from "@/lib/utils";
import { filtrosStore } from "@/stores/filtros.store";
import { mapActions, mapState } from "pinia";
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
  methods: {
    ...mapActions(filtrosStore, ["setMunicipio"]),
  },
  watch: {
    municipioId(val) {
      this.setMunicipio(val);
    },
    provincia_id() {
      this.municipioId = null;
    },
  },
  computed: {
    ...mapState(filtrosStore, ["provincia_id", "municipios", "provincia_id"]),
  },
  data: () => ({
    municipioId: null,
  }),
};
</script>

<template>
  <Select v-model="municipioId" :disabled="!provincia_id">
    <SelectTrigger class="w-45">
      <SelectValue placeholder="Selecciona un municipio" />
    </SelectTrigger>
    <SelectContent>
      <SelectGroup>
        <SelectLabel>Municipios</SelectLabel>
        <SelectItem v-for="municipio in municipios" :value="municipio.id">
          {{ municipio.nombre }}
        </SelectItem>
      </SelectGroup>
    </SelectContent>
  </Select>
</template>
