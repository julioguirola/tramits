<script setup lang="ts">
import { useSidebar } from "@/components/ui/sidebar";
import { onMounted, onBeforeUnmount, ref } from "vue";

const { isMobile, setOpenMobile } = useSidebar();
const el = ref<HTMLElement | null>(null);

function cerrar() {
  if (isMobile.value) {
    setOpenMobile(false);
  }
}

onMounted(() => {
  el.value?.parentElement?.addEventListener("click", cerrar);
});

onBeforeUnmount(() => {
  el.value?.parentElement?.removeEventListener("click", cerrar);
});
</script>

<template>
  <span ref="el" hidden />
</template>
