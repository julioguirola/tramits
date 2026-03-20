<script lang="ts">
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ArrowRightLeft } from "lucide-vue-next";

export default {
  components: {
    Button,
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    ArrowRightLeft,
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
      <Button> Confirmar Solicitud de Baja </Button>
    </CardContent>
  </Card>
</template>
