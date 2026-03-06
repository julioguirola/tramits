<script lang="ts">
import "vue-sonner/style.css";
import { Toaster } from "vue-sonner";
import SidebarProvider from "./components/ui/sidebar/SidebarProvider.vue";
import { SidebarTrigger } from "./components/ui/sidebar";
// import { api } from "./lib/utils";

export default {
  components: {
    Toaster,
    SidebarProvider,
    SidebarTrigger,
  },
};
</script>

<template>
  <SidebarProvider>
    <SidebarTrigger />
    <Toaster />
    <RouterView />
  </SidebarProvider>
</template>

<style scoped></style>
