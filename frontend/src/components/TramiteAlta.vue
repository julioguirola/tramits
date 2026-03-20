<script lang="ts">
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ArrowUpFromLine } from "lucide-vue-next";

export default {
  components: {
    Button,
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    ArrowUpFromLine,
  },
};
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle class="flex items-center gap-2">
        <ArrowUpFromLine class="size-4 text-muted-foreground" />
        Solicitar Alta
      </CardTitle>
      <CardDescription>
        Selecciona el núcleo al que deseas solicitar tu alta.
      </CardDescription>
    </CardHeader>
    <CardContent class="space-y-4">
      <Button> Solicitar Alta </Button>
    </CardContent>
  </Card>
</template>
