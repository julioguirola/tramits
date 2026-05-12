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
  Field,
  FieldDescription,
  FieldGroup,
  FieldLabel,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { api } from "@/lib/utils";

import Spinner from "@/components/ui/spinner/Spinner.vue";

export default {
  props: {
    uuid: { type: String, required: true },
  },
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    Button,
    Field,
    FieldDescription,
    FieldGroup,
    FieldLabel,
    Input,
    Spinner,
  },
  data() {
    return {
      password: "",
      confirmPassword: "",
      loading: false,
      success: false,
    };
  },
  computed: {
    valido(): boolean {
      return (
        this.password.length >= 8 && this.password === this.confirmPassword
      );
    },
  },
  methods: {
    async submit() {
      if (!this.valido || this.loading) return;
      this.loading = true;
      try {
        const res = await api.post(
          `/password-recovery/restablecer/${this.uuid}`,
          { password: this.password },
        );
        if (res?.status === 200) {
          this.success = true;
        }
      } finally {
        this.loading = false;
      }
    },
  },
};
</script>

<template>
  <div class="flex min-h-screen items-center justify-center px-4">
    <Card class="w-full max-w-md shadow-xl shadow-emerald-100/50 border-emerald-100/80">
      <CardHeader>
        <CardTitle>Restablecer contraseña</CardTitle>
        <CardDescription>
          Ingresa tu nueva contraseña.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div v-if="success" class="space-y-4">
          <p class="text-sm text-green-600 font-medium">
            Contraseña restablecida correctamente.
          </p>
          <Button class="w-full" @click="$router.push('/')">
            Iniciar sesión
          </Button>
        </div>
        <form v-else @submit.prevent="submit">
          <FieldGroup>
            <Field>
              <FieldLabel for="password">Nueva contraseña</FieldLabel>
              <Input
                id="password"
                type="password"
                required
                v-model="password"
                placeholder="Mínimo 8 caracteres"
              />
            </Field>
            <Field>
              <FieldLabel for="confirm-password">Confirmar contraseña</FieldLabel>
              <Input
                id="confirm-password"
                type="password"
                required
                v-model="confirmPassword"
                placeholder="Repite la contraseña"
              />
            </Field>
            <Field>
              <Button
                class="w-full"
                :disabled="!valido || loading"
                type="submit"
              >
                <Spinner v-if="loading" />
                {{ loading ? "" : "Restablecer contraseña" }}
              </Button>
            </Field>
          </FieldGroup>
        </form>
      </CardContent>
    </Card>
  </div>
</template>
