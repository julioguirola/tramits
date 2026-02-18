import { createRouter, createWebHistory } from "vue-router";
import Login from "@/Login.vue";
import Register from "@/Register.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      component: Login,
    },
    {
      path: "/register",
      component: Register,
    },
  ],
});

export default router;
