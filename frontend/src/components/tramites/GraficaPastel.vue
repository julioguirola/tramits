<script lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  ChartContainer,
  ChartTooltip,
} from "@/components/ui/chart";
import {
  VisDonut,
  VisSingleContainer,
} from "@unovis/vue";
import { Donut } from "@unovis/ts";
import type { ChartConfig } from "@/components/ui/chart";

const pieColors = [
  "#3b82f6",
  "#22c55e",
  "#f59e0b",
  "#ef4444",
  "#8b5cf6",
  "#06b6d4",
  "#ec4899",
  "#84cc16",
  "#f97316",
  "#6366f1",
];

export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    ChartContainer,
    ChartTooltip,
    VisDonut,
    VisSingleContainer,
  },
  props: {
    tiposData: {
      type: Array as () => { nombre: string; count: number }[],
      required: true,
    },
    chartConfig: {
      type: Object as () => ChartConfig,
      required: true,
    },
  },
  computed: {
    displayData() {
      return this.tiposData.slice(0, 10);
    },
    pieColorsByIndex() {
      return this.displayData.map((_, i) => pieColors[i % pieColors.length]);
    },
  },
  methods: {
    donutTriggers() {
      const self = this;
      return {
        [Donut.selectors.segment]: (_d: any, i: number) => {
          const d = _d?.data || _d;
          if (!d) return null;
          const nombre = d.nombre || d.name || "";
          const count = Number(d.count || d.value || 0);
          const total = self.displayData.reduce((sum, t) => sum + (Number(t.count) || 0), 0);
          if (total === 0) return `<div class="p-2">Sin datos</div>`;
          const percentage = Math.round((count / total) * 100);
          const color = self.pieColorsByIndex[i] || self.pieColorsByIndex[0];
          return `<div class="border-border/50 bg-background grid min-w-[12rem] items-start gap-2 rounded-lg border px-3 py-2.5 text-xs shadow-xl">
          <div class="flex items-center justify-between gap-2 pb-1.5 border-b border-border/50">
            <span class="truncate max-w-[10rem] font-semibold">${nombre}</span>
            <div class="w-3 h-3 rounded-full shrink-0" style="background-color: ${color}"></div>
          </div>
          <div class="grid gap-2 pt-1">
            <div class="flex justify-between items-center">
              <span class="text-muted-foreground flex items-center gap-1.5">
                <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2"/>
                  <rect x="9" y="3" width="6" height="4" rx="1"/>
                </svg>
                Trámites
              </span>
              <span class="text-foreground font-mono font-bold text-sm tabular-nums">${count.toLocaleString()}</span>
            </div>
            <div class="w-full bg-muted rounded-full h-2 mt-1">
              <div class="h-2 rounded-full transition-all" style="width: ${percentage}%; background-color: ${color}"></div>
            </div>
            <div class="text-center text-muted-foreground text-[10px]">
              ${percentage}% del total
            </div>
          </div>
        </div>`;
        },
      };
    },
  },
};
</script>

<template>
  <Card>
    <CardHeader class="pb-2">
      <CardTitle class="text-lg">Top Tipos de Trámite</CardTitle>
      <CardDescription>Los 10 tipos más solicitados</CardDescription>
    </CardHeader>
    <CardContent>
      <div class="flex gap-4">
        <div class="w-2/5 flex items-center justify-center">
          <ChartContainer
            :config="chartConfig"
            class="h-56 w-full"
            :cursor="true"
          >
            <VisSingleContainer
              :data="displayData"
              :height="224"
            >
              <VisDonut
                :value="(d: any) => d.count"
                :label="(d: any) => d.nombre"
                :color="(d: any, i: any) => pieColorsByIndex[i]"
                :arc-width="30"
                :corner-radius="3"
              />
              <ChartTooltip :triggers="donutTriggers()" />
            </VisSingleContainer>
          </ChartContainer>
        </div>
        <div class="w-3/5 flex flex-col">
          <div class="border rounded-lg bg-muted/20 p-2 flex-1 max-h-56 overflow-y-auto">
            <div
              v-for="(tipo, index) in displayData"
              :key="tipo.nombre"
              class="flex items-center gap-2 py-1 px-1 rounded hover:bg-muted/50 transition-colors"
            >
              <div
                class="w-2 h-2 rounded-full shrink-0"
                :style="{ backgroundColor: pieColorsByIndex[index] }"
              ></div>
              <span class="truncate text-xs flex-1">{{ tipo.nombre }}</span>
              <span class="text-xs font-medium text-muted-foreground tabular-nums shrink-0">
                {{ tipo.count.toLocaleString() }}
              </span>
            </div>
          </div>
          <div class="pt-2 text-xs text-muted-foreground text-center border-t mt-2">
            Total: {{ displayData.reduce((sum, t) => sum + t.count, 0).toLocaleString() }} trámites
          </div>
        </div>
      </div>
    </CardContent>
  </Card>
</template>