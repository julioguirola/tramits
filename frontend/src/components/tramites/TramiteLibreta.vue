<script lang="ts">
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
import DialogClose from "@/components/ui/dialog/DialogClose.vue";
import Spinner from "@/components/ui/spinner/Spinner.vue";
import { BookText } from "lucide-vue-next";
import { api } from "@/lib/utils";

export default {
  props: {
    bloqueado: {
      type: Boolean,
      default: false,
    },
    motivoBloqueo: {
      type: String,
      default: "",
    },
  },
  emits: ["created"],
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
    Spinner,
    BookText,
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
  methods: {
    async confirmarLibreta() {
      this.loading_solicitar = true;
      let res = await api.post("/tramite/libreta");
      if (res.status === 201) {
        this.dialog_open = false;
        this.$emit("created");
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
        <BookText class="size-4 text-muted-foreground" />
        Solicitar Libreta
      </CardTitle>
      <CardDescription>
        Solicita una nueva libreta de abastecimiento.
      </CardDescription>
    </CardHeader>
    <CardContent class="space-y-4">
      <Dialog v-model:open="dialog_open">
        <DialogTrigger as-child>
          <Button :disabled="bloqueado"> Solicitar Libreta </Button>
        </DialogTrigger>

        <DialogContent class="sm:max-w-106.25">
          <DialogHeader>
            <DialogTitle>Confirmar Solicitud de Libreta</DialogTitle>
            <DialogDescription>
              Esta acción iniciará el proceso de solicitud de una nueva libreta
              de abastecimiento.
            </DialogDescription>
          </DialogHeader>

          <DialogFooter>
            <DialogClose as-child>
              <Button variant="outline" :disabled="loading_solicitar">
                Cancelar
              </Button>
            </DialogClose>
            <Button @click="confirmarLibreta" :disabled="loading_solicitar">
              <Spinner v-if="loading_solicitar" />
              {{ loading_solicitar ? "" : "Confirmar Solicitud" }}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
      <p v-if="bloqueado" class="text-sm text-muted-foreground">
        {{ motivoBloqueo }}
      </p>
    </CardContent>
  </Card>
</template>
