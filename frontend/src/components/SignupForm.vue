<script lang="ts">
import { Button } from "@/components/ui/button";
import { toast } from "vue-sonner";
import { Spinner } from "@/components/ui/spinner";
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
import { api, toast_trigger } from "@/lib/utils";
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
    Spinner,
  },
  data() {
    return {
      email: "",
      carnet: "",
      pass_word: "",
      pass_word_c: "",
      persona: null,
      loading_verificar: false,
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
    async verificar_carnet(e: Event) {
      e.preventDefault();
      this.loading_verificar = true;
      try {
        const res = (
          await api.get("/persona", {
            params: { carnet: this.carnet },
          })
        ).data;
        toast_trigger(res);
        this.persona = res.data[0];
      } catch (e: any) {
        const res = e.response.data;
        if (res) {
          toast_trigger(res);
        }
      } finally {
        this.loading_verificar = false;
      }
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
          <div class="flex gap-4">
            <Field>
              <FieldLabel for="carnet"> Carnet </FieldLabel>
              <Input
                id="carnet"
                type="text"
                placeholder="03090660960"
                required
                v-model="carnet"
              />
            </Field>
            <Button @click="verificar_carnet" class="self-end">
              <Spinner v-if="loading_verificar" />
              {{ loading_verificar ? "" : "Verificar" }}
            </Button>
          </div>
          <p>{{ persona }}</p>
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
