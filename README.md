<div style="width: 100%; display: flex; justify-content: center;">
  <h1 style="display: inline-flex; align-items: center; gap: 10px; margin: 0 0 12px 0;">
    <img src="frontend/public/favicon.ico" alt="Tramits" width="32" height="32" style="vertical-align: middle;" />
    <span>Tramits</span>
  </h1>
</div>

![Tramits](frontend/public/tramites.webp)

**Plataforma web para la solicitud de trámites de núcleos familiares en Cuba.**

Este proyecto elimina la necesidad de presentarse presencialmente en las oficinas del Ministerio de Comercio Interior (MINCIN). Los ciudadanos pueden solicitar alta o baja de núcleos familiares desde cualquier lugar, mientras los registradores procesan las solicitudes de forma digital.

## Problema que resuelve

Anteriormente, el proceso de alta o baja de núcleos familiares en Cuba requería que el ciudadano se presentara personalmente en la bodega correspondiente. Este proyecto permite:

- **Solicitar trámites online** desde cualquier dispositivo con acceso a internet
- **Reducir la carga presencial** en las oficinas de comercio interior
- **Centralizar la gestión** de trámites para el MINCIN
- **Generar informes y estadísticas** para la toma de decisiones

## Stack

| Capa | Tecnología |
|------|-----------|
| Backend | Rust · Axum · SQLx · PostgreSQL |
| Frontend | Vue 3 · Pinia · shadcn-vue · Tailwind v4 · Bun |
| Cache | Redis (deadpool-redis) |
| Runtime | Docker |

## Arquitectura

```
tramits/
├── backend/          # API REST (Rust + Axum)
│   └── src/
│       ├── config/   # middlewares: auth, cache, cors, logger
│       ├── db/       # init_db, migration.sql, seed
│       ├── http/     # handlers HTTP
│       └── repos/    # lógica de negocio y acceso a BD
├── frontend/         # SPA (Vue 3 + Bun)
│   └── src/
│       ├── components/
│       ├── composables/    # lógica reutilizable (export PDF, etc)
│       ├── stores/         # Pinia
│       └── router/
├── docker-compose.dev.yml
└── docker-compose.prod.yml
```

## Modelo de datos

Jerarquía geográfica:
```
Provincia (15 en Cuba)
  └── Municipio (168+ en Cuba)
        └── Oficina
              └── Bodega
                    └── Núcleo (dirección del ciudadano)
```

Entidades principales:
- **persona**: Ciudadano cubano (nombre, apellido, CI de 11 dígitos)
- **usuario**: Credenciales del sistema con rol (Consumidor/Registrador/Administrador)
- **tramite**: Solicitud con tipo (Alta/Baja) y estado (Pendiente/En proceso/Completado/Rechazado/Cancelado)

## Roles de usuario

| Rol | Descripción |
|-----|-------------|
| Consumidor | Solicita trámites de alta/baja de núcleo familiar |
| Registrador | Gestiona y procesa solicitudes asignadas a su oficina |
| Administrador | Administra usuarios y configura el sistema |

## Funcionalidades

### Para Consumidores
- Solicitud de alta de núcleo familiar
- Solicitud de baja de núcleo familiar
- Historial de trámites personales
- Cancelación de trámites pendientes
- Exportación de datos a PDF/Excel

### Para Registradores
- Vista de trámites asignados a su oficina
- Procesamiento de solicitudes (Marcar en proceso → Completar/Rechazar)
- Estadísticas de gestión

### Para Administradores
- Gestión de usuarios
- Estadísticas generales del sistema
- Exportación de informes

### Estadísticas y Reportes
- Gráficos interactivos de trámites por mes (barras)
- Distribución por tipo de trámite (pastel/donut)
- Exportación a PDF con `modern-screenshot` + `jsPDF`
- Exportación a Excel con `xlsx`

## Desarrollo local

### Requisitos

- Docker y Docker Compose
- Bun (para el frontend)
- Copiar `.env.example` a `.env` y completar los valores

```bash
cp .env.example .env
```

### Variables de entorno requeridas

| Variable | Descripción |
|----------|-------------|
| `DB_HOST` / `DB_USER` / `DB_PASSWORD` / `DB_NAME` / `DB_PORT` | Conexión a PostgreSQL |
| `JWT_SECRET` | Secreto para firmar tokens JWT |
| `REDIS_URL` | URL del pool Redis (ej. `redis://cache:6379`) |
| `PORT` | Puerto del backend (ej. `3030`) |
| `SPA_URL` | Origen permitido en CORS (ej. `http://localhost:5173`) |
| `VITE_API_URL` | Base URL de la API para el frontend |
| `ADMIN_EMAIL` / `ADMIN_PASSWORD` | Credenciales del administrador inicial |
| `CONSUMER_EMAIL` / `CONSUMER_PASSWORD` | Credenciales del consumidor de prueba |
| `REGISTRAR_EMAIL` / `REGISTRAR_PASSWORD` | Credenciales del registrador de prueba |
| `MIGRATE` | `true` ejecuta migration + seed al iniciar |
| `ENVIRONMENT` | Nombre del entorno (dev/prod) |
| `RUST_LOG` | Nivel de logging (ej. `TRACE`) |

### Levantar

```bash
docker compose -f docker-compose.dev.yml --env-file .env up --build -d
```

Con `MIGRATE=true` la base de datos se inicializa automáticamente con datos de prueba y los tres usuarios seed.

### Puertos en dev

| Servicio | Puerto |
|----------|--------|
| Frontend | `5173` |
| Backend | `3030` |
| PostgreSQL | `5433` |
| Redis | `6377` |
| RedisInsight | `5540` |

## Comandos

### Frontend (siempre usar bun, no npm)

```bash
cd frontend
bun install        # instalar dependencias
bun run dev        # servidor de desarrollo
bun run build      # type-check + build
bun run type-check # solo vue-tsc
```

### Backend

```bash
cd backend
cargo check        # verificar errores
cargo clippy       # linter
cargo fmt          # formatear
cargo test         # tests
cargo run --bin migration  # ejecutar migraciones
cargo run --bin seeder     # poblar datos de prueba
```

## Producción

En el servidor solo se necesita el `docker-compose.prod.yml` y un `.env.prod`. Para obtener el compose:

```bash
curl -o docker-compose.prod.yml https://raw.githubusercontent.com/julioguirola/tramits/master/docker-compose.prod.yml
```

Levantar:

```bash
docker compose -f docker-compose.prod.yml --env-file .env.prod up -d
```

## API — Endpoints principales

| Método | Ruta | Auth | Descripción |
|--------|------|------|-------------|
| `POST` | `/usuario` | No | Registrar usuario |
| `POST` | `/login` | No | Iniciar sesión (JWT en cookie HttpOnly) |
| `POST` | `/logout` | Sí | Cerrar sesión |
| `GET` | `/usuario/me` | Sí + cache | Datos del usuario autenticado |
| `GET` | `/tramite/estadisticas` | Sí | Estadísticas (Registrador/Administrador) |
| `POST` | `/tramite/:id/procesar` | Sí | Marcar trámite como "En proceso" |
| `POST` | `/tramite/:id/gestionar` | Sí | Completar o rechazar trámite |
| `POST` | `/tramite/:id/cancelar` | Sí | Cancelar trámite (solo Consumidor, solo Pendiente) |