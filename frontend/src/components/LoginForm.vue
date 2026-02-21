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
import { RouterLink } from "vue-router";
export default {
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
    RouterLink,
  },
  data(): {
    email: string;
    pass_word: string;
  } {
    return {
      email: "",
      pass_word: "",
    };
  },
  mounted() {},
};
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Inicia sesión en tu cuenta</CardTitle>
      <CardDescription>
        Ingresa tu correo electrónico a continuación para iniciar sesión en tu
        cuenta
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
            <Button> Iniciar sesión </Button>
            <FieldDescription class="text-center">
              ¿No tienes una cuenta?
              <RouterLink to="/register">Regístrate</RouterLink>
            </FieldDescription>
          </Field>
        </FieldGroup>
      </form>
    </CardContent>
  </Card>
</template>
