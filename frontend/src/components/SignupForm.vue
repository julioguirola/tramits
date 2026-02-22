<script lang="ts">
import { Button } from "@/components/ui/button";
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
import {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
} from "@/components/ui/hover-card";
import { Input } from "@/components/ui/input";
import { api, toast_trigger } from "@/lib/utils";
import PersonaCard from "@/components/PersonaCard.vue";
import router from "@/router";
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
    PersonaCard,
    HoverCard,
    HoverCardContent,
    HoverCardTrigger,
  },
  data(): {
    email: string;
    carnet: string;
    pass_word: string;
    persona?: {
      id: string;
      nombre: string;
      apellido: string;
      carnet: string;
      direccion: string;
    };
    loading_verificar: boolean;
    loading_crear: boolean;
  } {
    return {
      email: "",
      carnet: "",
      pass_word: "",
      persona: undefined,
      loading_verificar: false,
      loading_crear: false,
    };
  },
  methods: {
    async submit(e: Event) {
      e.preventDefault();
      try {
        this.loading_crear = true;
        const res = (
          await api.post("/usuario", {
            email: this.email,
            pass_word: this.pass_word,
            persona_id: this.persona?.id,
          })
        ).data;
        toast_trigger(res);
        router.push("/dashboard");
      } catch (e: any) {
        const res = e.response.data;
        if (res) {
          toast_trigger(res);
        }
      } finally {
        this.loading_crear = false;
      }
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
        this.persona = undefined;
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
            <Button
              @click="verificar_carnet"
              class="self-end"
              :disabled="loading_verificar"
            >
              <Spinner v-if="loading_verificar" />
              {{ loading_verificar ? "" : "Verificar" }}
            </Button>
          </div>
          <PersonaCard v-if="persona" :persona="persona" />
          <HoverCard>
            <HoverCardTrigger>
              <Field>
                <FieldLabel for="email"> Email </FieldLabel>
                <Input
                  id="email"
                  type="email"
                  placeholder="m@example.com"
                  required
                  v-model="email"
                  :disabled="persona == undefined"
                />
                <FieldDescription>
                  Usaremos esta información para contactarte. No compartiremos
                  tu correo electrónico con nadie más.
                </FieldDescription>
              </Field>
            </HoverCardTrigger>
            <HoverCardContent v-if="persona == undefined" class="bg-[#cdffd9]"
              >Introduzca su carnet primero</HoverCardContent
            >
          </HoverCard>
          <HoverCard>
            <HoverCardTrigger>
              <Field>
                <FieldLabel for="password"> Contraseña </FieldLabel>
                <Input
                  id="password"
                  type="password"
                  placeholder="ej: t64a#:-+xfWCwMx"
                  required
                  v-model="pass_word"
                  :disabled="persona == undefined"
                />
                <FieldDescription
                  >Debe tener al menos 8 caracteres.</FieldDescription
                >
              </Field>
            </HoverCardTrigger>
            <HoverCardContent v-if="persona == undefined" class="bg-[#cdffd9]"
              >Introduzca su carnet primero</HoverCardContent
            >
          </HoverCard>
          <FieldGroup>
            <HoverCard
              ><HoverCardTrigger
                ><Field>
                  <Button
                    @click="submit"
                    :disabled="
                      persona == undefined ||
                      !email ||
                      !pass_word ||
                      loading_crear
                    "
                  >
                    <Spinner v-if="loading_crear" />
                    {{ loading_crear ? "" : "Crear Cuenta" }}
                  </Button>
                  <FieldDescription class="px-6 text-center">
                    ¿Ya tienes una cuenta?
                    <RouterLink to="/">Iniciar sesión</RouterLink>
                  </FieldDescription>
                </Field>
                <HoverCardContent class="bg-[#cdffd9]"
                  >Completa los campos anteriores</HoverCardContent
                ></HoverCardTrigger
              ></HoverCard
            >
          </FieldGroup>
        </FieldGroup>
      </form>
    </CardContent>
  </Card>
</template>
