# AGENTS.md

## Project Overview

## Preferencias del agente

- Solo hacer cambios.
- No conversar ni explicar nada, salvo cuando este en modo plan.

**Tramits** es una plataforma web para la gestión de trámites de núcleos familiares en Cuba. Permite a los ciudadanos (consumidores) solicitar alta o baja de núcleos de forma online, eliminando la necesidad de presentarse presencialmente en las oficinas.

**Organización destino**: Ministerio de Comercio Interior de Cuba (MINCIN)

## Arquitectura de la base de datos

Jerarquía geográfica:
```
Provincia (15 en Cuba)
  └── Municipio (168+ en Cuba)
        └── Oficina (puede haber varias por municipio)
              └── Bodega (una o más por oficina)
                    └── Núcleo (dirección específica donde vive la persona)
```

**Entidades principales**:
- `persona`: Ciudadano cubano con nombre, apellido, CI (11 dígitos) y nucleus_id opcional
- `usuario`: Usuario del sistema (email, password, rol) vinculado a persona
- `tramite`: Solicitud con tipo (Alta/Baja), estado (Pendiente/En proceso/Completado/Rechazado/Cancelado)
- Roles: Consumidor (crea trámites), Registrador (procesa trámites), Administrador (gestiona usuarios)

## Repo layout

- `backend/` - API REST en Rust con Axum, 3 binaries: `main`, `migration`, `seeder`
- `frontend/` - SPA en Vue 3 + Bun (NUNCA npm, siempre bun)

## Frontend (Bun - CRÍTICO)

**siempre usar bun, no npm/pnpm/yarn**

```bash
cd frontend
bun i              # instalar dependencias (importante: usar 'i', no 'install')
bun run dev        # desarrollo con Vite
bun run build      # VERIFICAR build: type-check + vite build (siempre ejecutar antes de commit)
bun run type-check # solo validación de tipos
```

> **Importante**: Verificar que `bun run build` pasa sin errores antes de hacer commit.

Paquetes destacados instalados:
- `@unovis/vue` + `@unovis/ts` - Gráficos interactivos
- `modern-screenshot` + `jspdf` - Exportación a PDF
- `xlsx` - Exportación a Excel
- `pinia` - State management
- `reka-ui` - Componentes UI (shadcn-vue style)

## Backend (Rust)

```bash
cd backend
cargo check        # verificar tipos
cargo clippy       # linter
cargo fmt          # formatear
cargo run --bin migration  # ejecutar migraciones
cargo run --bin seeder     # poblar datos de prueba
```

## Local dev

```bash
docker compose -f docker-compose.dev.yml --env-file .env up --build -d
```

- Dev compose usa Dockerfiles en minúsculas: `backend/dockerfile` y `frontend/dockerfile`
- Backend: `cargo run` en el container
- Frontend: `bun run dev` con bind mounts para `src/`, `public/`, `vite.config.ts`, `components.json`, `index.html`

## Variables de entorno requeridas

`.env.example` es la plantilla; backend hace panic si falta alguna de `backend/src/config/mod.rs`

- `DB_HOST/DB_USER/DB_PASSWORD/DB_NAME/DB_PORT` - PostgreSQL
- `POSTGRES_*` en docker-compose.dev.yml mapea a `DB_*` del backend
- `JWT_SECRET`, `REDIS_URL`, `PORT`, `SPA_URL`, `VITE_API_URL`
- `MIGRATE=true` ejecuta migration + seed al iniciar

## Key commands

- Frontend build: `bun run build` -> `vue-tsc --build` + `vite build`
- Backend migration/seeder: binaries separados (`cargo run --bin migration/seeder`)

## Roles de usuario

| Rol | ID | Descripción |
|-----|----|-------------|
| Consumidor | 1 | Solicita trámites de alta/baja |
| Registrador | 2 | Procesa trámites asignados a su oficina |
| Administrador | 3 | Gestiona usuarios del sistema |

## Workflow de trámites

1. Consumidor crea solicitud (tipo: Alta o Baja)
2. Registrador ve trámites pendientes de su oficina
3. Registrador procesa -> estado pasa a "En proceso"
4. Registrador completa/rechaza -> estado final
5. Consumidor puede cancelar solo si está Pendiente

## Componentes de estadísticas (frontend)

Ubicación: `frontend/src/components/tramites/GraficaPastel.vue` y `GraficaBarras.vue`

- Usan Unovis para gráficos
- Tooltips con información enriquecida
- Lazy loading con `defineAsyncComponent` + `<Suspense>`
- Exportación PDF con `modern-screenshot` + `jspdf`

## Backend Response Format

All API responses use the `Ress<T>` struct:

```rust
pub struct Ress<T> {
    pub message: &'static str,   // "Éxito", "Error", "Atención", "Información"
    pub description: &'static str,  // Human-readable message
    pub data: Option<T>,         // Actual payload
}
```

**Message types**:
| Message | Toast Type | Use Case |
|---------|-----------|----------|
| `"Éxito"` | `toast.success()` | Operation completed successfully |
| `"Error"` | `toast.error()` | Operation failed |
| `"Atención"` | `toast.warning()` | Validation issues, business rule warnings |
| `"Información"` | `toast.info()` | Informational messages |

**Example response**:
```json
{
  "message": "Éxito",
  "description": "Trámite procesado correctamente",
  "data": { "id": "uuid-here" }
}
```

## Frontend Toast Notifications

The frontend uses `vue-sonner` for toast notifications. Toast handling is **automático** via axios interceptor.

**Setup**:
- `Toaster` component is registered in `App.vue`
- `toast_trigger()` function in `frontend/src/lib/utils.ts` handles all responses
- Axios interceptor at `api.interceptors.response` calls `toast_trigger()` on every response

**How it works**:
```typescript
// frontend/src/lib/utils.ts
api.interceptors.response.use(
  (res) => {
    toast_trigger(res.data);  // Auto-show toast on success
    return res;
  },
  (err: AxiosError) => {
    if (err.status == 401) {
      router.push("/");  // Redirect to login on auth failure
    }
    if (err.response) {
      toast_trigger(err.response.data);  // Auto-show toast on error
    }
    return err.response;
  },
);
```

**Toast trigger function**:
```typescript
enum Respuesta {
  Success = "Éxito",
  Error = "Error",
  Warn = "Atención",
  Info = "Información",
}

export function toast_trigger(res: any) {
  switch (res.message) {
    case Respuesta.Success:
      toast.success(Respuesta.Success, { description: res.description });
      break;
    case Respuesta.Error:
      toast.error(Respuesta.Error, { description: res.description });
      break;
    case Respuesta.Warn:
      toast.warning(Respuesta.Warn, { description: res.description });
      break;
    case Respuesta.Info:
      toast.info(Respuesta.Info, { description: res.description });
      break;
    default:
      toast.error(Respuesta.Error, { description: res });
  }
}
```

## CI / Production

- Workflow en `.github/workflows/deploy.yml` construye y sube imágenes Docker
- `VITE_API_URL` se inyecta via build-arg
