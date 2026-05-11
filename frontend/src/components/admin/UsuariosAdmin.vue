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
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
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
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Download, Loader2, Mail, Users, UserCog } from "lucide-vue-next";
import * as XLSX from "xlsx";
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
  oficina_id: number | null;
  municipio: string;
  provincia: string;
  activo: boolean;
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
    Input,
    Label,
    Pagination,
    PaginationContent,
    PaginationEllipsis,
    PaginationFirst,
    PaginationItem,
    PaginationLast,
    PaginationNext,
    PaginationPrevious,
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
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
    Download,
    Mail,
    UserCog,
  },
  data() {
    return {
      usuarios: [] as UsuarioListado[],
      cargando: false,
      enviandoCorreo: false,
      actualizandoEstado: null as string | null,
      page: 1,
      limit: 10,
      total: 0,
      limitOptions: [5, 10, 20, 30],
      filtrosKey: 0,
      dialogCorreoAbierto: false,
      correoUsuario: null as UsuarioListado | null,
      correoAsunto: "",
      correoCuerpo: "",
      dialogDesactivarAbierto: false,
      desactivarUsuario: null as UsuarioListado | null,
      motivoDesactivacion: "",
      dialogRolAbierto: false,
      rolUsuario: null as UsuarioListado | null,
      actualizandoRol: false,
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
    hayFiltrosActivos(): boolean {
      return Boolean(this.provincia_id || this.municipio_id || this.oficina_id);
    },
    tieneUsuarios(): boolean {
      return this.usuarios.length > 0;
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
    async limpiarFiltros() {
      const filtros = filtrosStore();
      filtros.$patch({
        provincia_id: null,
        municipio_id: null,
        oficina_id: null,
        municipios: [],
        oficinas: [],
      });
      this.filtrosKey += 1;
      this.page = 1;
      await this.cargarUsuarios();
    },
    exportarUsuariosExcel() {
      if (this.usuarios.length === 0) return;
      const data = this.usuarios.map((usuario) => ({
        Nombre: `${usuario.nombre} ${usuario.apellido}`,
        Email: usuario.email,
        Rol: usuario.rol,
        Oficina: usuario.oficina ?? "-",
        Municipio: usuario.municipio,
        Provincia: usuario.provincia,
      }));
      const ws = XLSX.utils.json_to_sheet(data);
      const wb = XLSX.utils.book_new();
      XLSX.utils.book_append_sheet(wb, ws, "Usuarios");
      XLSX.writeFile(
        wb,
        `usuarios_${new Date().toISOString().split("T")[0]}.xlsx`,
      );
    },
    abrirCorreo(usuario: UsuarioListado) {
      this.correoUsuario = usuario;
      this.correoAsunto = "";
      this.correoCuerpo = "";
      this.dialogCorreoAbierto = true;
    },
    async enviarCorreo() {
      if (!this.correoUsuario || this.enviandoCorreo) return;
      const asunto = this.correoAsunto.trim();
      const cuerpo = this.correoCuerpo.trim();
      if (!asunto || !cuerpo) return;

      this.enviandoCorreo = true;
      try {
        const res = await api.post("/usuarios/correo", {
          usuario_id: this.correoUsuario.id,
          asunto,
          cuerpo,
        });
        if (res?.status === 200) {
          this.dialogCorreoAbierto = false;
        }
      } finally {
        this.enviandoCorreo = false;
      }
    },
    abrirDialogDesactivar(usuario: UsuarioListado) {
      this.desactivarUsuario = usuario;
      this.motivoDesactivacion = "";
      this.dialogDesactivarAbierto = true;
    },
    async activarUsuario(usuario: UsuarioListado) {
      if (this.actualizandoEstado) return;
      this.actualizandoEstado = usuario.id;
      try {
        const res = await api.post("/usuarios/estado", {
          usuario_id: usuario.id,
          activo: true,
        });
        if (res?.status === 200) {
          usuario.activo = true;
        }
      } finally {
        this.actualizandoEstado = null;
      }
    },
    async confirmarDesactivar() {
      if (!this.desactivarUsuario || this.actualizandoEstado) return;
      const motivo = this.motivoDesactivacion.trim();
      if (!motivo) return;

      this.actualizandoEstado = this.desactivarUsuario.id;
      try {
        const res = await api.post("/usuarios/estado", {
          usuario_id: this.desactivarUsuario.id,
          activo: false,
          motivo,
        });
        if (res?.status === 200) {
          this.desactivarUsuario.activo = false;
          this.dialogDesactivarAbierto = false;
          this.desactivarUsuario = null;
        }
      } finally {
        this.actualizandoEstado = null;
      }
    },
    abrirDialogRol(usuario: UsuarioListado) {
      const filtros = filtrosStore();
      this.rolUsuario = usuario;
      filtros.$patch({
        provincia_id: null,
        municipio_id: null,
        oficina_id: null,
        municipios: [],
        oficinas: [],
        bodegas: [],
      });
      this.dialogRolAbierto = true;
    },
    async actualizarRol() {
      if (!this.rolUsuario || this.actualizandoRol) return;
      this.actualizandoRol = true;
      const oficinaId = this.rolUsuario.rol === "Consumidor" ? this.oficina_id ?? null : null;
      if (this.rolUsuario.rol === "Consumidor" && !oficinaId) {
        this.actualizandoRol = false;
        return;
      }
      try {
        const res = await api.post("/usuarios/rol", {
          usuario_id: this.rolUsuario.id,
          oficina_id: oficinaId,
        });
        if (res?.status === 200) {
          this.rolUsuario.oficina_id = oficinaId;
          this.rolUsuario.rol = oficinaId ? "Registrador" : "Consumidor";
          if (!oficinaId) {
            this.rolUsuario.oficina = null;
          }
          this.dialogRolAbierto = false;
        }
      } finally {
        this.actualizandoRol = false;
      }
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
      <CardTitle class="flex flex-wrap items-center justify-between gap-3">
        <span class="flex items-center gap-2">
          <Users class="size-5" />
          Usuarios del sistema
        </span>
        <Button
          v-if="tieneUsuarios"
          size="sm"
          variant="outline"
          class="gap-2"
          @click="exportarUsuariosExcel"
        >
          <Download class="size-4" />
          Exportar
        </Button>
      </CardTitle>
      <CardDescription>
        {{ total }} usuarios registrados · Mostrando {{ mostrando }}
      </CardDescription>
      <div class="flex flex-wrap gap-3 pt-2" :key="filtrosKey">
        <ProvinciaFiltro />
        <MunicipioFiltro />
        <OficinaFiltro />
        <Button v-if="hayFiltrosActivos" variant="outline" @click="limpiarFiltros">
          Limpiar filtros
        </Button>
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
              <TableHead>Estado</TableHead>
              <TableHead class="text-right">Acciones</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-if="cargando">
              <TableCell colspan="8" class="text-center">
                <Loader2 class="inline-block size-5 animate-spin text-muted-foreground" />
              </TableCell>
            </TableRow>
            <TableRow v-else-if="usuarios.length === 0">
              <TableCell colspan="8" class="text-center text-muted-foreground">
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
              <TableCell>
                <Button
                  size="sm"
                  :variant="usuario.activo ? 'destructive' : 'default'"
                  :disabled="actualizandoEstado === usuario.id"
                  @click="usuario.activo ? abrirDialogDesactivar(usuario) : activarUsuario(usuario)"
                  :title="usuario.activo ? 'Desactivar usuario' : 'Activar usuario'"
                >
                  <Loader2
                    v-if="actualizandoEstado === usuario.id"
                    class="mr-2 size-4 animate-spin"
                  />
                  {{ usuario.activo ? "Desactivar" : "Activar" }}
                </Button>
              </TableCell>
              <TableCell class="text-right">
                <div class="flex justify-end gap-2">
                  <Button
                    size="icon"
                    variant="ghost"
                    class="text-muted-foreground hover:text-foreground"
                    @click="abrirDialogRol(usuario)"
                    title="Modificar rol"
                  >
                    <UserCog class="size-4" />
                  </Button>
                  <Button
                    size="icon"
                    variant="ghost"
                    class="text-muted-foreground hover:text-foreground"
                    @click="abrirCorreo(usuario)"
                    title="Enviar correo"
                  >
                    <Mail class="size-4" />
                  </Button>
                </div>
              </TableCell>
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

  <Dialog v-model:open="dialogCorreoAbierto">
    <DialogContent class="max-w-lg">
      <DialogHeader>
        <DialogTitle>Enviar correo</DialogTitle>
        <DialogDescription>
          Escribe el asunto y el cuerpo del mensaje para
          {{ correoUsuario?.nombre }} {{ correoUsuario?.apellido }}.
        </DialogDescription>
      </DialogHeader>
      <div class="space-y-4">
        <div class="space-y-2">
          <Label for="correo-asunto">Asunto</Label>
          <Input
            id="correo-asunto"
            v-model="correoAsunto"
            placeholder="Ej. Actualización de cuenta"
          />
        </div>
        <div class="space-y-2">
          <Label for="correo-cuerpo">Cuerpo</Label>
          <textarea
            id="correo-cuerpo"
            v-model="correoCuerpo"
            class="flex min-h-32 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-xs outline-none transition-[color,box-shadow] placeholder:text-muted-foreground focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50"
            rows="6"
            placeholder="Escribe el contenido del correo"
          ></textarea>
        </div>
      </div>
      <DialogFooter class="gap-2">
        <DialogClose as-child>
          <Button variant="outline" :disabled="enviandoCorreo">Cancelar</Button>
        </DialogClose>
        <Button
          class="gap-2"
          :disabled="enviandoCorreo || !correoAsunto.trim() || !correoCuerpo.trim()"
          @click="enviarCorreo"
        >
          <Loader2 v-if="enviandoCorreo" class="size-4 animate-spin" />
          Enviar
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>

  <Dialog v-model:open="dialogDesactivarAbierto">
    <DialogContent class="max-w-lg">
      <DialogHeader>
        <DialogTitle>Desactivar usuario</DialogTitle>
        <DialogDescription>
          Indica la causa de la desactivación para notificar al usuario.
        </DialogDescription>
      </DialogHeader>
      <div class="space-y-2">
        <Label for="motivo-desactivacion">Causa</Label>
        <textarea
          id="motivo-desactivacion"
          v-model="motivoDesactivacion"
          class="flex min-h-28 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-xs outline-none transition-[color,box-shadow] placeholder:text-muted-foreground focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50"
          rows="5"
          placeholder="Explica la causa de la desactivación"
        ></textarea>
      </div>
      <DialogFooter class="gap-2">
        <DialogClose as-child>
          <Button variant="outline" :disabled="actualizandoEstado !== null">Cancelar</Button>
        </DialogClose>
        <Button
          variant="destructive"
          class="gap-2"
          :disabled="
            actualizandoEstado !== null || !motivoDesactivacion.trim()
          "
          @click="confirmarDesactivar"
        >
          <Loader2 v-if="actualizandoEstado !== null" class="size-4 animate-spin" />
          Desactivar cuenta
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>

  <Dialog v-model:open="dialogRolAbierto">
    <DialogContent class="max-w-lg">
      <DialogHeader>
        <DialogTitle>Modificar rol</DialogTitle>
        <DialogDescription>
          Gestiona el rol del usuario seleccionado.
        </DialogDescription>
      </DialogHeader>
      <div v-if="rolUsuario?.rol === 'Registrador'" class="space-y-3">
        <p class="text-sm text-muted-foreground">
          Este usuario ya es registrador. ¿Deseas pasarlo a consumidor?
        </p>
      </div>
      <div v-else class="space-y-2">
        <Label for="rol-oficina">Oficina</Label>
        <div class="flex flex-wrap gap-3">
          <ProvinciaFiltro />
          <MunicipioFiltro />
          <OficinaFiltro />
        </div>
        <div class="text-xs text-muted-foreground">
          Selecciona una provincia, municipio y oficina para asignarlo como
          registrador.
        </div>
      </div>
      <DialogFooter class="gap-2">
        <Button
          variant="outline"
          :disabled="actualizandoRol"
          @click="dialogRolAbierto = false"
        >
          Cancelar
        </Button>
        <Button
          class="gap-2"
          :disabled="
            actualizandoRol ||
            (rolUsuario?.rol === 'Consumidor' && !oficina_id)
          "
          @click="actualizarRol"
        >
          <Loader2 v-if="actualizandoRol" class="size-4 animate-spin" />
          Cambiar rol
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
