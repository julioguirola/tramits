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
import { GalleryVerticalEnd } from "lucide-vue-next";
import { Home } from "lucide-vue-next";
import DashboardUser from "./components/DashboardUser.vue";
import { api } from "@/lib/utils";
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
    GalleryVerticalEnd,
    Home,
    DashboardUser,
  },
  data() {
    return {
      user: {
        nombre: "",
        apellido: "",
        email: "",
        tipo: "",
        avatar: "https://github.com/julioguirola.png",
      },
    };
  },
  async mounted() {
    const res = await api.get("/usuario/me");
    if (res?.status === 200 && res.data?.data) {
      const d = res.data.data;
      this.user.nombre = d.nombre;
      this.user.apellido = d.apellido;
      this.user.email = d.email;
      this.user.tipo = d.tipo;
    }
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
                <span class="truncate font-semibold">Acme Inc</span>
                <span class="truncate text-xs">Enterprise</span>
              </div>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Platform</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton as-child>
                  <RouterLink to="/">
                    <Home />
                    <span>Home</span>
                  </RouterLink>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <DashboardUser :user="user" />
      </SidebarFooter>
    </Sidebar>
    <SidebarInset>
      <header
        class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12"
      >
        <div class="flex items-center gap-2 px-4">
          <SidebarTrigger class="-ml-1 hover:cursor-pointer" />
        </div>
      </header>
      <RouterView />
    </SidebarInset>
  </SidebarProvider>
</template>
