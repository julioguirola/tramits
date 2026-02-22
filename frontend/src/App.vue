<script lang="ts">
import "vue-sonner/style.css";
import { Toaster } from "vue-sonner";
// import { api } from "./lib/utils";

export default {
  components: {
    Toaster,
  },
  data() {
    return {
      name: "",
    };
  },
  mounted() {
    // api.get("/").then((d) => (this.name = d.data));
    // api.post("/", { sdf: "fsd" });
    // fetch("http://localhost:3030/", {
    //   method: "POST",
    //   body: JSON.stringify({ sdf: "fsd" }),
    //   headers: {
    //     "Content-Type": "application/json",
    //   },
    // });
  },
};
</script>

<template>
  <!-- {{ name }} -->
  <Toaster />
  <RouterView />
</template>

<style scoped></style>
