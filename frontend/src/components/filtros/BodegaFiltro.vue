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
    ...mapActions(filtrosStore, ["setBodega"]),
  },
  watch: {
    bodegaId(val) {
      this.setBodega(val);
    },
    oficina_id() {
      this.bodegaId = null;
    },
  },
  computed: {
    ...mapState(filtrosStore, ["oficina_id", "bodegas"]),
  },
  data: () => ({
    bodegaId: null,
  }),
};
</script>

<template>
  <Select v-model="bodegaId" :disabled="!oficina_id">
    <SelectTrigger class="w-45">
      <SelectValue placeholder="Selecciona una bodega" />
    </SelectTrigger>
    <SelectContent>
      <SelectGroup>
        <SelectLabel>Bodegas</SelectLabel>
        <SelectItem
          v-for="bodega in bodegas"
          :value="bodega.id"
          :key="bodega.id"
        >
          {{ bodega.nombre }}
        </SelectItem>
      </SelectGroup>
    </SelectContent>
  </Select>
</template>
