<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { cn } from "@/lib/utils";
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

const props = defineProps<{
  class?: HTMLAttributes["class"];
}>();
</script>

<template>
  <div :class="cn('flex flex-col gap-6', props.class)">
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
              <Input id="password" type="password" required />
            </Field>
            <Field>
              <Button type="submit"> Iniciar sesión </Button>
              <Button variant="outline" type="button">
                Iniciar sesión con Google
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
  </div>
</template>
