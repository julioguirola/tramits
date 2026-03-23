<script lang="ts">
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarRail,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import {
  GalleryVerticalEnd,
  FilePlus,
  ClipboardList,
  Users,
  UserRound,
} from "lucide-vue-next";
import DashboardUser from "./components/DashboardUser.vue";
import SidebarMobileClose from "./components/SidebarMobileClose.vue";
import { useUsuarioStore } from "@/stores/usuario.store";
import { mapState, mapActions } from "pinia";

export default {
  components: {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarHeader,
    SidebarInset,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarProvider,
    SidebarRail,
    SidebarTrigger,
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
    GalleryVerticalEnd,
    DashboardUser,
    SidebarMobileClose,
  },
  computed: {
    ...mapState(useUsuarioStore, ["usuario"]),
    menuItems() {
      const tipo = this.usuario?.rol;
      if (tipo === "Consumidor") {
        return [
          {
            label: "Mi informacion",
            icon: this.icons.UserRound,
            to: "/dashboard/mi-informacion",
          },
          {
            label: "Nuevo tramite",
            icon: this.icons.FilePlus,
            to: "/dashboard/nuevo-tramite",
          },
        ];
      }
      if (tipo === "Registrador") {
        return [
          {
            label: "Solicitudes",
            icon: this.icons.ClipboardList,
            to: "/dashboard/tramites",
          },
        ];
      }
      if (tipo === "Administrador") {
        return [
          {
            label: "Usuarios",
            icon: this.icons.Users,
            to: "/dashboard/usuarios",
          },
        ];
      }
      return [];
    },
    breadcrumb(): string | null {
      const path = this.$route.path;
      const segment = path.replace(/^\/dashboard\/?/, "");
      if (!segment) return null;
      const labels: Record<string, string> = {
        "mi-informacion": "Mi informacion",
        "nuevo-tramite": "Nuevo tramite",
        tramites: "Solicitudes",
        usuarios: "Usuarios",
      };
      return labels[segment] ?? segment;
    },
  },
  data() {
    return {
      icons: { FilePlus, ClipboardList, Users, UserRound },
    };
  },
  methods: {
    ...mapActions(useUsuarioStore, ["cargar"]),
  },
  async mounted() {
    await this.cargar();
  },
};
</script>

<template>
  <SidebarProvider>
    <Sidebar collapsible="icon" variant="floating">
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg">
              <div
                class="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground"
              >
                <GalleryVerticalEnd class="size-4" />
              </div>
              <div class="grid flex-1 text-left text-sm leading-tight">
                <span class="truncate font-semibold">Tramits</span>
                <span class="truncate text-xs"
                  >Solicitud de Trámites Online</span
                >
              </div>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Tramits</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem v-for="item in menuItems" :key="item.to">
                <SidebarMenuButton as-child>
                  <RouterLink :to="item.to">
                    <component :is="item.icon" />
                    <span>{{ item.label }}</span>
                    <SidebarMobileClose />
                  </RouterLink>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <DashboardUser v-if="usuario" :user="usuario" />
      </SidebarFooter>
    </Sidebar>
    <SidebarInset class="flex flex-col min-h-screen">
      <header
        class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12"
      >
        <div class="flex items-center gap-2 px-4">
          <SidebarTrigger class="-ml-1 hover:cursor-pointer" />
          <template v-if="breadcrumb">
            <Breadcrumb>
              <BreadcrumbList>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                  <BreadcrumbPage>{{ breadcrumb }}</BreadcrumbPage>
                </BreadcrumbItem>
              </BreadcrumbList>
            </Breadcrumb>
          </template>
        </div>
      </header>
      <div class="flex-1">
        <RouterView />
      </div>
    </SidebarInset>
  </SidebarProvider>
</template>
