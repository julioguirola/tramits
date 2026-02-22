import { createRouter, createWebHistory } from "vue-router";
import Login from "@/Login.vue";
import Register from "@/Register.vue";
import Dashboard from "@/Dashboard.vue";
import NotFound from "@/components/NotFound.vue";

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
    {
      path: "/dashboard",
      component: Dashboard,
    },
    {
      path: "/:pathMatch(.*)*",
      component: NotFound,
    },
  ],
});

export default router;
