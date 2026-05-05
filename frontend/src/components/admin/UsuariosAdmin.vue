<script lang="ts">
import { api } from "@/lib/utils";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationFirst,
  PaginationItem,
  PaginationLast,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Loader2, Users } from "lucide-vue-next";
import { filtrosStore } from "@/stores/filtros.store";
import { mapState } from "pinia";
import ProvinciaFiltro from "@/components/filtros/ProvinciaFiltro.vue";
import MunicipioFiltro from "@/components/filtros/MunicipioFiltro.vue";
import OficinaFiltro from "@/components/filtros/OficinaFiltro.vue";

interface UsuarioListado {
  id: string;
  email: string;
  nombre: string;
  apellido: string;
  rol: string;
  oficina: string | null;
  municipio: string;
  provincia: string;
}

interface UsuariosResponse {
  usuarios: UsuarioListado[];
  total: number;
  page: number;
  limit: number;
}

export default {
  components: {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
    Button,
    Pagination,
    PaginationContent,
    PaginationEllipsis,
    PaginationFirst,
    PaginationItem,
    PaginationLast,
    PaginationNext,
    PaginationPrevious,
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectTrigger,
    SelectValue,
    Loader2,
    Users,
    ProvinciaFiltro,
    MunicipioFiltro,
    OficinaFiltro,
  },
  data() {
    return {
      usuarios: [] as UsuarioListado[],
      cargando: false,
      page: 1,
      limit: 10,
      total: 0,
      limitOptions: [5, 10, 20, 30],
    };
  },
  computed: {
    ...mapState(filtrosStore, ["provincia_id", "municipio_id", "oficina_id"]),
    totalPages(): number {
      return Math.max(1, Math.ceil(this.total / this.limit));
    },
    mostrando(): string {
      if (this.total === 0) return "0";
      const start = (this.page - 1) * this.limit + 1;
      const end = Math.min(this.page * this.limit, this.total);
      return `${start}-${end}`;
    },
  },
  methods: {
    async cargarUsuarios() {
      if (this.cargando) return;
      this.cargando = true;
      try {
        const res = await api.get("/usuarios", {
          params: {
            page: this.page,
            limit: this.limit,
            provincia_id: this.provincia_id ?? undefined,
            municipio_id: this.municipio_id ?? undefined,
            oficina_id: this.oficina_id ?? undefined,
          },
        });
        if (res?.status === 200 && res.data?.data) {
          const data = res.data.data as UsuariosResponse;
          this.usuarios = data.usuarios || [];
          this.total = data.total ?? 0;
          this.page = data.page ?? this.page;
          this.limit = data.limit ?? this.limit;
        }
      } finally {
        this.cargando = false;
      }
    },
    async irAPagina(page: number) {
      if (this.cargando || page === this.page || page < 1 || page > this.totalPages) return;
      this.page = page;
      await this.cargarUsuarios();
    },
    async cambiarLimite(limite: number) {
      if (this.cargando || this.limit === limite) return;
      this.limit = limite;
      this.page = 1;
      await this.cargarUsuarios();
    },
  },
  watch: {
    provincia_id() {
      this.page = 1;
      this.cargarUsuarios();
    },
    municipio_id() {
      this.page = 1;
      this.cargarUsuarios();
    },
    oficina_id() {
      this.page = 1;
      this.cargarUsuarios();
    },
  },
  async mounted() {
    await this.cargarUsuarios();
  },
};
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle class="flex items-center gap-2">
        <Users class="size-5" />
        Usuarios del sistema
      </CardTitle>
      <CardDescription>
        {{ total }} usuarios registrados · Mostrando {{ mostrando }}
      </CardDescription>
      <div class="flex flex-wrap gap-3 pt-2">
        <ProvinciaFiltro />
        <MunicipioFiltro />
        <OficinaFiltro />
      </div>
    </CardHeader>
    <CardContent>
      <div class="w-full overflow-x-auto">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Nombre</TableHead>
              <TableHead>Email</TableHead>
              <TableHead>Rol</TableHead>
              <TableHead>Oficina</TableHead>
              <TableHead>Municipio</TableHead>
              <TableHead>Provincia</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-if="cargando">
              <TableCell colspan="6" class="text-center">
                <Loader2 class="inline-block size-5 animate-spin text-muted-foreground" />
              </TableCell>
            </TableRow>
            <TableRow v-else-if="usuarios.length === 0">
              <TableCell colspan="6" class="text-center text-muted-foreground">
                No hay usuarios registrados.
              </TableCell>
            </TableRow>
            <TableRow v-else v-for="usuario in usuarios" :key="usuario.id">
              <TableCell class="font-medium">
                {{ usuario.nombre }} {{ usuario.apellido }}
              </TableCell>
              <TableCell>{{ usuario.email }}</TableCell>
              <TableCell>{{ usuario.rol }}</TableCell>
              <TableCell>{{ usuario.oficina ?? "-" }}</TableCell>
              <TableCell>{{ usuario.municipio }}</TableCell>
              <TableCell>{{ usuario.provincia }}</TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </div>
      <div
        class="mt-6 flex flex-col gap-4 border-t pt-4 sm:flex-row sm:items-center sm:justify-between"
      >
        <div class="flex flex-wrap items-center gap-4">
          <div class="text-sm text-muted-foreground">
            Página {{ page }} de {{ totalPages }}
          </div>
          <div class="flex items-center gap-2 text-sm">
            <span class="text-muted-foreground">Tamaño de página</span>
            <Select
              :model-value="String(limit)"
              @update:model-value="(value) => cambiarLimite(Number(value))"
            >
              <SelectTrigger class="w-20 h-8">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectLabel>Resultados</SelectLabel>
                  <SelectItem
                    v-for="option in limitOptions"
                    :key="option"
                    :value="String(option)"
                  >
                    {{ option }}
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
        </div>
        <Pagination
          :page="page"
          :total="total"
          :sibling-count="1"
          :items-per-page="limit"
          @update:page="irAPagina"
        >
          <PaginationContent v-slot="{ items }">
            <PaginationFirst />
            <PaginationPrevious />
            <template
              v-for="(item, index) in items"
              :key="item.type + '-' + index"
            >
              <PaginationItem
                v-if="item.type === 'page'"
                :value="item.value"
                :is-active="item.value === page"
              >
                {{ item.value }}
              </PaginationItem>
              <PaginationEllipsis v-else-if="item.type === 'ellipsis'" />
            </template>
            <PaginationNext />
            <PaginationLast />
          </PaginationContent>
        </Pagination>
      </div>
    </CardContent>
  </Card>
</template>
