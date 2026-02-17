<script lang="ts">
export default {
  data() {
    return {
      name: "Juan",
    };
  },
  async mounted() {
    fetch("http://localhost:6006/")
      .then((r) => r.text())
      .then((d) => (this.name = d));
  },
};
</script>

<template>
  {{ name }}
</template>

<style scoped></style>
