<script lang="ts">
import { useUsuarioStore } from "@/stores/usuario.store";
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
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { ArrowRightLeft } from "lucide-vue-next";
import DialogClose from "@/components/ui/dialog/DialogClose.vue";
import Spinner from "@/components/ui/spinner/Spinner.vue";
import { api } from "@/lib/utils";

export default {
  components: {
    Button,
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
    DialogClose,
    ArrowRightLeft,
    Spinner,
  },
  data(): {
    loading_solicitar: boolean;
    dialog_open: boolean;
  } {
    return {
      loading_solicitar: false,
      dialog_open: false,
    };
  },
  computed: {
    ...mapState(useUsuarioStore, ["usuario"]),
    nucleoActual(): string | null {
      return this.usuario?.nucleo || null;
    },
  },
  methods: {
    async confirmarBaja() {
      this.loading_solicitar = true;
      let res = await api.post("/tramite/baja", {
        nucleo_id: this.usuario?.numero_nucleo,
      });
      if (res.status === 201) {
        this.dialog_open = false;
      }
      this.loading_solicitar = false;
    },
  },
};
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle class="flex items-center gap-2">
        <ArrowRightLeft class="size-4 text-muted-foreground" />
        Solicitar Baja
      </CardTitle>
      <CardDescription> Solicita la baja de tu núcleo actual. </CardDescription>
    </CardHeader>
    <CardContent class="space-y-4">
      <Dialog v-model:open="dialog_open">
        <DialogTrigger as-child>
          <Button> Solicitar Baja </Button>
        </DialogTrigger>

        <DialogContent class="sm:max-w-106.25">
          <DialogHeader>
            <DialogTitle>Confirmar Solicitud de Baja</DialogTitle>
            <DialogDescription>
              Esta acción iniciará el proceso de baja de tu núcleo actual.
            </DialogDescription>
          </DialogHeader>

          <DialogFooter>
            <DialogClose as-child>
              <Button variant="outline" :disabled="loading_solicitar">
                Cancelar
              </Button>
            </DialogClose>
            <Button @click="confirmarBaja" :disabled="loading_solicitar">
              <Spinner v-if="loading_solicitar" />
              {{ loading_solicitar ? "" : "Confirmar Baja" }}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </CardContent>
  </Card>
</template>
