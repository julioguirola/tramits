import { createRouter, createWebHistory } from "vue-router";
import Login from "@/Login.vue";
import Register from "@/Register.vue";
import Dashboard from "@/Dashboard.vue";
import NotFound from "@/components/NotFound.vue";
import axios from "axios";

const authCheck = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
  withCredentials: true,
});

const guestOnly = async () => {
  try {
    const res = await authCheck.get("/usuario/me");
    if (res.status === 200) return "/dashboard";
  } catch {
    return true;
  }
};

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      component: Login,
      beforeEnter: guestOnly,
    },
    {
      path: "/register",
      component: Register,
      beforeEnter: guestOnly,
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
