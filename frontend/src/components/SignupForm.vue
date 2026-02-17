<script setup lang="ts">
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
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Crea una cuenta</CardTitle>
      <CardDescription>
        Completa los siguientes campos con tu información
      </CardDescription>
    </CardHeader>
    <CardContent>
      <form>
        <FieldGroup>
          <Field>
            <FieldLabel for="email"> Email </FieldLabel>
            <Input
              id="email"
              type="email"
              placeholder="m@example.com"
              required
            />
            <FieldDescription>
              Usaremos esta información para contactarte. No compartiremos tu
              correo electrónico con nadie más.
            </FieldDescription>
          </Field>
          <Field>
            <FieldLabel for="password"> Contraseña </FieldLabel>
            <Input id="password" type="password" required />
            <FieldDescription
              >Debe tener al menos 8 caracteres.</FieldDescription
            >
          </Field>
          <Field>
            <FieldLabel for="confirm-password">
              Confirmar Contraseña
            </FieldLabel>
            <Input id="confirm-password" type="password" required />
            <FieldDescription
              >Por favor confirme su contraseña.</FieldDescription
            >
          </Field>
          <FieldGroup>
            <Field>
              <Button type="submit"> Crear Cuenta </Button>
              <FieldDescription class="px-6 text-center">
                ¿Ya tienes una cuenta? <a href="#">Iniciar sesión</a>
              </FieldDescription>
            </Field>
          </FieldGroup>
        </FieldGroup>
      </form>
    </CardContent>
  </Card>
</template>
