import { createRouter, createWebHistory } from "vue-router";
import Login from "../Login.vue";
import Register from "@/Register.vue";
import Dashboard from "@/Dashboard.vue";
import Bienvenida from "@/components/dashboard/Bienvenida.vue";
import UsuariosAdmin from "@/components/admin/UsuariosAdmin.vue";
import MiInformacion from "@/components/persona/MiInformacion.vue";
import NuevoTramite from "@/components/tramites/NuevoTramite.vue";
import Solicitudes from "@/components/tramites/Solicitudes.vue";
import HistorialRegistrador from "@/components/tramites/HistorialRegistrador.vue";
import Estadisticas from "@/components/tramites/Estadisticas.vue";
import NotFound from "@/components/common/NotFound.vue";
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
      children: [
        { path: "", component: Bienvenida },
        { path: "mi-informacion", component: MiInformacion },
        { path: "nuevo-tramite", component: NuevoTramite },
        { path: "tramites", component: Solicitudes },
        { path: "historial", component: HistorialRegistrador },
        { path: "estadisticas", component: Estadisticas },
        { path: "usuarios", component: UsuariosAdmin },
      ],
    },
    {
      path: "/:pathMatch(.*)*",
      component: NotFound,
    },
  ],
});

export default router;
