<script lang="ts">
import { useUsuarioStore } from "@/stores/usuario.store";
import { mapState } from "pinia";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Home, Store, Building2, MapPin, Map } from "lucide-vue-next";

export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  },
  data() {
    return {
      icons: { Home, Store, Building2, MapPin, Map },
    };
  },
  computed: {
    ...mapState(useUsuarioStore, ["usuario"]),
    campos(): { label: string; valor: string | null; icon: object }[] {
      if (!this.usuario) return [];
      return [
        {
          label: "Nucleo",
          valor: this.usuario.nucleo + " # " + this.usuario.numero_nucleo,
          icon: this.icons.Home,
        },
        { label: "Bodega", valor: this.usuario.bodega, icon: this.icons.Store },
        {
          label: "Oficina",
          valor: this.usuario.oficina,
          icon: this.icons.Building2,
        },
        {
          label: "Municipio",
          valor: this.usuario.municipio,
          icon: this.icons.MapPin,
        },
        {
          label: "Provincia",
          valor: this.usuario.provincia,
          icon: this.icons.Map,
        },
      ];
    },
  },
};
</script>

<template>
  <div class="flex flex-col gap-6 p-6 w-fit md:self-start self-center">
    <Card>
      <CardHeader>
        <CardTitle>Mi información</CardTitle>
        <CardDescription>
          Datos del núcleo familiar y ubicación asociados a tu cuenta.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div class="flex flex-col md:flex-row gap-4">
          <div
            v-for="item in campos"
            :key="item.label"
            class="flex items-center gap-3 rounded-lg border p-3"
          >
            <component
              :is="item.icon"
              class="size-5 shrink-0 text-muted-foreground"
            />
            <div class="flex flex-col">
              <span class="text-xs text-muted-foreground">{{
                item.label
              }}</span>
              <span class="text-sm font-medium">{{
                item.valor ?? "Sin asignar"
              }}</span>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
