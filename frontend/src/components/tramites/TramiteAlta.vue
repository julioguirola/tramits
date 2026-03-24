<script lang="ts">
import { api } from "@/lib/utils";
import { filtrosStore } from "@/stores/filtros.store";
import { mapState } from "pinia";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import ProvinciaFiltro from "@/components/filtros/ProvinciaFiltro.vue";
import MunicipioFiltro from "@/components/filtros/MunicipioFiltro.vue";
import OficinaFiltro from "@/components/filtros/OficinaFiltro.vue";
import BodegaFiltro from "@/components/filtros/BodegaFiltro.vue";
import { ArrowUpFromLine, Loader2 } from "lucide-vue-next";

interface Nucleo {
  id: number;
  direccion: string;
}

export default {
  components: {
    Button,
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectTrigger,
    SelectValue,
    ProvinciaFiltro,
    MunicipioFiltro,
    OficinaFiltro,
    BodegaFiltro,
    ArrowUpFromLine,
    Loader2,
  },
  data(): {
    open: boolean;
    loading_crear: boolean;
    nucleos: Nucleo[];
    nucleo_id: number | null;
  } {
    return {
      open: false,
      loading_crear: false,
      nucleos: [],
      nucleo_id: null,
    };
  },
  computed: {
    ...mapState(filtrosStore, ["bodega_id"]),
  },
  watch: {
    async bodega_id(val) {
      if (val) {
        await this.cargarNucleos();
      } else {
        this.nucleos = [];
        this.nucleo_id = null;
      }
    },
  },
  methods: {
    async cargarNucleos() {
      if (!this.bodega_id) return;
      try {
        const res = await api.get("/nucleo", {
          params: {
            bodega_id: this.bodega_id,
          },
        });
        if (res?.status === 200) {
          this.nucleos = res.data.data || [];
        }
      } catch (error) {
        console.error("Error cargando núcleos:", error);
      }
    },
    async solicitarAlta() {
      if (!this.nucleo_id) return;

      this.loading_crear = true;
      try {
        const res = await api.post("/tramite/alta", {
          nucleo_id: this.nucleo_id,
        });

        if (res?.status === 201) {
          this.open = false;
          this.nucleo_id = null;
        }
      } catch (error) {
        console.error("Error solicitando alta:", error);
      } finally {
        this.loading_crear = false;
      }
    },
  },
};
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle class="flex items-center gap-2">
        <ArrowUpFromLine class="size-4 text-muted-foreground" />
        Solicitar Alta
      </CardTitle>
      <CardDescription>
        Selecciona el núcleo al que deseas solicitar tu alta.
      </CardDescription>
    </CardHeader>
    <CardContent class="space-y-4">
      <Dialog v-model:open="open">
        <DialogTrigger as-child>
          <Button> Solicitar Alta </Button>
        </DialogTrigger>
        <DialogContent class="max-w-md">
          <DialogHeader>
            <DialogTitle>Solicitar Alta</DialogTitle>
            <DialogDescription>
              Selecciona el núcleo al que deseas unirte. Completa todos los
              filtros para elegir tu núcleo.
            </DialogDescription>
          </DialogHeader>

          <div class="space-y-4 py-4">
            <!-- Provincia -->
            <div class="space-y-2">
              <label class="text-sm font-medium">Provincia</label>
              <ProvinciaFiltro />
            </div>

            <!-- Municipio -->
            <div class="space-y-2">
              <label class="text-sm font-medium">Municipio</label>
              <MunicipioFiltro />
            </div>

            <!-- Oficina -->
            <div class="space-y-2">
              <label class="text-sm font-medium">Oficina</label>
              <OficinaFiltro />
            </div>

            <!-- Bodega -->
            <div class="space-y-2">
              <label class="text-sm font-medium">Bodega</label>
              <BodegaFiltro />
            </div>

            <!-- Núcleo -->
            <div class="space-y-2">
              <label class="text-sm font-medium">Núcleo</label>
              <Select v-model="nucleo_id" :disabled="!bodega_id">
                <SelectTrigger>
                  <SelectValue placeholder="Selecciona un núcleo" />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectLabel>Núcleos</SelectLabel>
                    <SelectItem
                      v-for="nucleo in nucleos"
                      :key="nucleo.id"
                      :value="nucleo.id"
                    >
                      {{ nucleo.direccion }}
                    </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
            </div>
          </div>

          <DialogFooter>
            <DialogClose as-child>
              <Button variant="outline">Cancelar</Button>
            </DialogClose>
            <Button
              @click="solicitarAlta"
              :disabled="!nucleo_id || loading_crear"
            >
              <Loader2 v-if="loading_crear" class="mr-2 size-4 animate-spin" />
              Confirmar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </CardContent>
  </Card>
</template>
