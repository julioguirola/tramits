<script lang="ts">
import "vue-sonner/style.css";
import { Toaster } from "vue-sonner";

export default {
  components: {
    Toaster,
  },
};
</script>

<template>
  <Toaster />
  <RouterView />
</template>

<style scoped></style>
