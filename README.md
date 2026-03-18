<div style="width: 100%; display: flex; justify-content: center;">
  <h1 style="display: inline-flex; align-items: center; gap: 10px; margin: 0 0 12px 0;">
    <img src="frontend/public/favicon.ico" alt="Tramits" width="32" height="32" style="vertical-align: middle;" />
    <span>Tramits</span>
  </h1>
</div>

![Tramits](frontend/public/tramites.webp)

Plataforma web para la solicitud de tráites de los consumidores en Cuba. Permite gestionar solicitudes de alta y baja de núcleos familiares, con roles diferenciados para consumidores, registradores y administradores.

## Stack

| Capa | Tecnología |
|------|-----------|
| Backend | Rust · Axum · SQLx · PostgreSQL |
| Frontend | Vue 3 · Pinia · shadcn-vue · Tailwind v4 |
| Cache | Redis (deadpool-redis) |
| Runtime | Docker · Bun |

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
│       ├── stores/        # Pinia
│       └── router/
├── docker-compose.dev.yml
└── docker-compose.prod.yml
```

## Roles de usuario

| Rol | ID | Acceso |
|-----|----|--------|
| Consumidor | 1 | Solicitar alta/baja de núcleo familiar |
| Registrador | 2 | Gestionar solicitudes de trámites |
| Administrador | 3 | Administrar usuarios del sistema |

## Desarrollo local

### Requisitos

- Docker y Docker Compose
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
| `GET` | `/nucleo` | Sí | Listar núcleos (`?page=1&limit=20&bodega_id=`) |

## Comandos útiles

```bash
# Backend
cargo check        # verificar errores
cargo clippy       # linter
cargo fmt          # formatear
cargo test         # tests

# Frontend
bun run dev        # servidor de desarrollo
bun run build      # type-check + build
bun run type-check # solo vue-tsc
```
