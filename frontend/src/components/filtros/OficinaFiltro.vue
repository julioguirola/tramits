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
  methods: {
    ...mapActions(filtrosStore, ["setOficina"]),
  },
  watch: {
    oficinaId(val) {
      this.setOficina(val);
    },
    municipio_id() {
      this.oficinaId = null;
    },
  },
  computed: {
    ...mapState(filtrosStore, ["municipio_id", "oficinas", "oficina_id"]),
  },
  data: () => ({
    oficinaId: filtrosStore().oficina_id,
  }),
};
</script>

<template>
  <Select v-model="oficinaId" :disabled="!municipio_id">
    <SelectTrigger class="w-45">
      <SelectValue placeholder="Selecciona una oficina" />
    </SelectTrigger>
    <SelectContent>
      <SelectGroup>
        <SelectLabel>Oficinas</SelectLabel>
        <SelectItem v-for="oficina in oficinas" :value="oficina.id">
          {{ oficina.nombre }}
        </SelectItem>
      </SelectGroup>
    </SelectContent>
  </Select>
</template>
