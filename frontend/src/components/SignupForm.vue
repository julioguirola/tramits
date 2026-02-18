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
export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    Field,
    FieldDescription,
    FieldGroup,
    FieldLabel,
    Input,
    Button,
  },
  data() {
    return {
      email: "",
      pass_word: "",
      pass_word_c: "",
    };
  },
  methods: {
    async submit(e: Event) {
      e.preventDefault();
      const res = (
        await api.post("/usuario", {
          email: this.email,
          pass_word: this.pass_word,
        })
      ).data;
      return;
    },
  },
};
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
              v-model="email"
            />
            <FieldDescription>
              Usaremos esta información para contactarte. No compartiremos tu
              correo electrónico con nadie más.
            </FieldDescription>
          </Field>
          <Field>
            <FieldLabel for="password"> Contraseña </FieldLabel>
            <Input id="password" type="password" required v-model="pass_word" />
            <FieldDescription
              >Debe tener al menos 8 caracteres.</FieldDescription
            >
          </Field>
          <Field>
            <FieldLabel for="confirm-password">
              Confirmar Contraseña
            </FieldLabel>
            <Input
              id="confirm-password"
              type="password"
              required
              v-model="pass_word_c"
            />
            <FieldDescription
              >Por favor confirme su contraseña.</FieldDescription
            >
          </Field>
          <FieldGroup>
            <Field>
              <Button @click="submit"> Crear Cuenta </Button>
              <FieldDescription class="px-6 text-center">
                ¿Ya tienes una cuenta?
                <RouterLink to="/">Iniciar sesión</RouterLink>
              </FieldDescription>
            </Field>
          </FieldGroup>
        </FieldGroup>
      </form>
    </CardContent>
  </Card>
</template>
