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
  ChartLegendContent,
  ChartTooltip,
  ChartCrosshair,
} from "@/components/ui/chart";
import {
  VisXYContainer,
  VisGroupedBar,
  VisAxis,
} from "@unovis/vue";
import type { ChartConfig } from "@/components/ui/chart";

export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    ChartContainer,
    ChartLegendContent,
    ChartTooltip,
    ChartCrosshair,
    VisXYContainer,
    VisGroupedBar,
    VisAxis,
  },
  props: {
    chartData: {
      type: Array as () => { mes: string; count: number }[],
      required: true,
    },
    chartConfig: {
      type: Object as () => ChartConfig,
      required: true,
    },
  },
  methods: {
    barTooltip() {
      return (_data: any, _x: number | Date) => {
        const data = "data" in _data ? _data.data : _data;
        if (!data) return null;
        const mes = data.mes || "";
        const count = data.count || 0;
        const monthNames: Record<string, string> = {
          "01": "Enero", "02": "Febrero", "03": "Marzo", "04": "Abril",
          "05": "Mayo", "06": "Junio", "07": "Julio", "08": "Agosto",
          "09": "Septiembre", "10": "Octubre", "11": "Noviembre", "12": "Diciembre",
        };
        const monthNum = mes.substring(0, 2);
        const year = mes.length > 4 ? mes.substring(3) : "";
        const monthName = monthNames[monthNum] || mes;
        const fullLabel = year ? `${monthName} ${year}` : monthName;
        const maxCount = Math.max(...(this.chartData.map(m => m.count) || [1]));
        const percentage = maxCount > 0 ? Math.round((count / maxCount) * 100) : 0;
        return `<div class="border-border/50 bg-background grid min-w-[10rem] items-start gap-2 rounded-lg border px-3 py-2 text-xs shadow-xl">
          <div class="flex items-center gap-2 font-semibold text-foreground pb-1 border-b border-border/50">
            <svg class="w-3 h-3 text-primary" viewBox="0 0 24 24" fill="currentColor">
              <rect x="3" y="12" width="4" height="9" rx="1"/>
              <rect x="10" y="8" width="4" height="13" rx="1"/>
              <rect x="17" y="4" width="4" height="17" rx="1"/>
            </svg>
            ${fullLabel}
          </div>
          <div class="grid gap-1.5 pt-1">
            <div class="flex justify-between items-center gap-4">
              <span class="text-muted-foreground flex items-center gap-1">
                <span class="inline-block w-2 h-2 rounded-full bg-primary"></span>
                Trámites
              </span>
              <span class="text-foreground font-mono font-semibold tabular-nums">${count.toLocaleString()}</span>
            </div>
            <div class="w-full bg-muted rounded-full h-1.5 mt-1">
              <div class="bg-primary h-1.5 rounded-full transition-all" style="width: ${percentage}%"></div>
            </div>
            <div class="text-center text-muted-foreground text-[10px]">
              ${percentage}% del mes con más trámites
            </div>
          </div>
        </div>`;
      };
    },
  },
};
</script>

<template>
  <Card class="mt-6">
    <CardHeader>
      <CardTitle>Trámites por Mes</CardTitle>
      <CardDescription>Distribución mensual de trámites</CardDescription>
    </CardHeader>
    <CardContent>
      <ChartContainer
        :config="chartConfig"
        class="h-120 w-full"
        :cursor="true"
      >
        <VisXYContainer :data="chartData">
          <VisGroupedBar
            :x="(d: any) => d.index"
            :y="(d: any) => d.count"
            :color="chartConfig?.tramites?.color || 'var(--chart-1)'"
            :rounded-corners="4"
            :bar-padding="0.1"
          />
          <VisAxis
            type="x"
            :x="(d: any) => d.index"
            :tick-format="(d: any) => chartData[d]?.mes || ''"
            :num-ticks="chartData.length"
            :tick-line="false"
            :domain-line="false"
            :grid-line="false"
          />
          <VisAxis
            type="y"
            :tick-format="(d: any) => Math.round(d)"
            :num-ticks="
              Math.min(10, Math.max(...chartData.map((d) => d.count)) + 1)
            "
            :tick-line="false"
            :domain-line="false"
            :grid-line="true"
          />
          <ChartTooltip />
          <ChartCrosshair :template="barTooltip()" />
        </VisXYContainer>
        <ChartLegendContent />
      </ChartContainer>
    </CardContent>
  </Card>
</template>