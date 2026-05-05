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
  async mounted() {
    await this.getProvincias();
  },
  methods: {
    ...mapActions(filtrosStore, ["setProvincia", "getProvincias"]),
  },
  watch: {
    provinciaId(val) {
      this.setProvincia(val);
    },
  },
  computed: {
    ...mapState(filtrosStore, ["provincias", "provincia_id"]),
  },
  data: () => ({
    provinciaId: filtrosStore().provincia_id,
  }),
};
</script>

<template>
  <Select v-model="provinciaId">
    <SelectTrigger class="w-45">
      <SelectValue placeholder="Selecciona una provincia" />
    </SelectTrigger>
    <SelectContent>
      <SelectGroup>
        <SelectLabel>Provincias</SelectLabel>
        <SelectItem v-for="provincia in provincias" :value="provincia.id">
          {{ provincia.nombre }}
        </SelectItem>
      </SelectGroup>
    </SelectContent>
  </Select>
</template>
