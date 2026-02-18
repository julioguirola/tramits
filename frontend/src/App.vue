<script lang="ts">
import { api } from "./lib/utils";

export default {
  data() {
    return {
      name: "",
    };
  },
  mounted() {
    api.get("/").then((d) => (this.name = d.data));
  },
};
</script>

<template>
  {{ name }}
  <RouterView />
</template>

<style scoped></style>
