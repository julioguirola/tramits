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
} from "@/components/ui/dialog";
import {
  Field,
  FieldDescription,
  FieldGroup,
  FieldLabel,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { api } from "@/lib/utils";
import router from "@/router";
import { RouterLink } from "vue-router";
import Spinner from "@/components/ui/spinner/Spinner.vue";
export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    Button,
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    Field,
    FieldDescription,
    FieldGroup,
    FieldLabel,
    Input,
    RouterLink,
    Spinner,
  },
  data(): {
    email: string;
    pass_word: string;
    loading_iniciar: boolean;
    dialogRecoveryOpen: boolean;
    recoveryEmail: string;
    loadingRecovery: boolean;
    recoverySent: boolean;
  } {
    return {
      email: "",
      pass_word: "",
      loading_iniciar: false,
      dialogRecoveryOpen: false,
      recoveryEmail: "",
      loadingRecovery: false,
      recoverySent: false,
    };
  },
  methods: {
    async submit() {
      this.loading_iniciar = true;
      const res = await api.post("/login", {
        email: this.email,
        pass_word: this.pass_word,
      });
      if (res?.status === 200) {
        router.push("/dashboard");
      }
      this.loading_iniciar = false;
    },
    abrirRecovery() {
      this.recoveryEmail = "";
      this.recoverySent = false;
      this.dialogRecoveryOpen = true;
    },
    async enviarRecovery() {
      if (!this.recoveryEmail.trim() || this.loadingRecovery) return;
      this.loadingRecovery = true;
      try {
        await api.post("/password-recovery/solicitar", {
          email: this.recoveryEmail.trim(),
        });
        this.recoverySent = true;
      } finally {
        this.loadingRecovery = false;
      }
    },
  },
};
</script>

<template>
  <Card class="w-full shadow-xl shadow-emerald-100/50 border-emerald-100/80">
    <CardHeader>
      <CardTitle>Inicia sesión en tu cuenta</CardTitle>
      <CardDescription>
        Ingresa tu correo electrónico a continuación para iniciar sesión en tu
        cuenta.
      </CardDescription>
    </CardHeader>
    <CardContent>
      <form>
        <FieldGroup>
          <Field>
            <FieldLabel for="email"> Correo electrónico </FieldLabel>
            <Input
              id="email"
              type="email"
              placeholder="m@ejemplo.com"
              required
              v-model="email"
            />
          </Field>
          <Field>
            <div class="flex items-center">
              <FieldLabel for="password"> Contraseña </FieldLabel>
              <a
                href="#"
                class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                @click.prevent="abrirRecovery"
              >
                ¿Olvidaste tu contraseña?
              </a>
            </div>
            <Input
              id="password"
              type="password"
              required
              v-model="pass_word"
              placeholder="ej: t64a#:-+xfWCwMx"
            />
          </Field>
          <Field>
            <Button
              :disabled="!email || !pass_word || loading_iniciar"
              @click="submit"
            >
              <Spinner v-if="loading_iniciar" />
              {{ loading_iniciar ? "" : "Iniciar sesión" }}
            </Button>
            <FieldDescription class="text-center">
              ¿No tienes una cuenta?
              <RouterLink to="/register">Regístrate</RouterLink>
            </FieldDescription>
          </Field>
        </FieldGroup>
      </form>
    </CardContent>
  </Card>

  <Dialog v-model:open="dialogRecoveryOpen">
    <DialogContent class="max-w-sm">
      <DialogHeader>
        <DialogTitle>Restablecer contraseña</DialogTitle>
        <DialogDescription>
          Ingresa tu correo electrónico y te enviaremos un enlace para
          restablecer tu contraseña.
        </DialogDescription>
      </DialogHeader>
      <div v-if="recoverySent" class="py-4 text-sm text-green-600 font-medium">
        Si el correo existe, recibirás un enlace para restablecer tu contraseña.
      </div>
      <div v-else class="space-y-4">
        <Field>
          <FieldLabel for="recovery-email">Correo electrónico</FieldLabel>
          <Input
            id="recovery-email"
            type="email"
            v-model="recoveryEmail"
            placeholder="m@ejemplo.com"
          />
        </Field>
      </div>
      <DialogFooter class="gap-2">
        <Button
          variant="outline"
          :disabled="loadingRecovery"
          @click="dialogRecoveryOpen = false"
        >
          Cancelar
        </Button>
        <Button
          v-if="!recoverySent"
          :disabled="!recoveryEmail.trim() || loadingRecovery"
          @click="enviarRecovery"
        >
          <Spinner v-if="loadingRecovery" />
          {{ loadingRecovery ? "" : "Enviar enlace" }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
