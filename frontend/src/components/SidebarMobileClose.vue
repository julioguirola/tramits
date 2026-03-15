<script setup lang="ts">
import { watch } from "vue";
import { useRoute } from "vue-router";
import { useSidebar } from "@/components/ui/sidebar";

const { isMobile, setOpenMobile } = useSidebar();
const route = useRoute();

watch(
  () => route.path,
  () => {
    if (isMobile.value) {
      setOpenMobile(false);
    }
  },
);
</script>

<template>
  <span hidden />
</template>
