# Guía de instalación y despliegue — Tramits

## Requisitos de hardware

### Desarrollo local

| Componente | Mínimo | Recomendado |
|------------|--------|-------------|
| RAM | 8 GB | 16 GB |
| CPU | 2 núcleos | 4 núcleos |
| Disco | 20 GB libres | 40 GB SSD |
| SO | Linux, macOS o Windows (WSL2) | Linux (Ubuntu 24.04+) |

### Servidor de producción

| Componente | Mínimo | Recomendado |
|------------|--------|-------------|
| RAM | 2 GB | 4 GB |
| CPU | 1 núcleo | 2 núcleos |
| Disco | 10 GB libres | 20 GB SSD |
| SO | Ubuntu 22.04+ / Debian 12+ | Ubuntu 24.04 LTS |
| Conexión | 10 Mbps | 100 Mbps |

## Requisitos de software

### Desarrollo local

| Software | Versión | Propósito |
|----------|---------|-----------|
| [Docker](https://docs.docker.com/engine/install/) | 24+ | Contenedores de backend, DB, Redis, MailHog |
| [Docker Compose](https://docs.docker.com/compose/install/) | v2+ | Orquestación de servicios |
| [Bun](https://bun.sh/docs/install) | 1.3+ | Runtime y build del frontend |
| [Rust](https://rustup.rs/) (opcional) | 1.83+ | Compilar y verificar el backend localmente |
| Git | 2.x | Control de versiones |

### Producción

| Software | Versión | Propósito |
|----------|---------|-----------|
| Docker | 24+ | Contenedores de todos los servicios |
| Docker Compose | v2+ | Orquestación |
| Nginx o Caddy (opcional) | — | Proxy inverso y TLS |

---

## 1. Estructura del proyecto

```
tramits/
├── backend/                   # API REST en Rust + Axum
│   ├── src/
│   │   ├── main.rs            # Punto de entrada, router
│   │   ├── config/mod.rs      # Variables de entorno
│   │   ├── db/                # init_db + migration.sql
│   │   ├── http/              # Handlers HTTP
│   │   ├── repos/             # Lógica de negocio
│   │   ├── mail/              # Envío de correos SMTP
│   │   ├── middlewares/        # auth, cors, logger
│   │   └── tipos/             # Tipos compartidos
│   ├── dockerfile
│   └── Cargo.toml
├── frontend/                  # SPA en Vue 3 + Bun
│   ├── src/
│   │   ├── components/        # Componentes Vue
│   │   ├── stores/            # Pinia stores
│   │   ├── router/            # Vue Router
│   │   └── lib/               # Utilidades
│   ├── dockerfile
│   └── package.json
├── docker-compose.dev.yml     # Entorno de desarrollo
├── docker-compose.prod.yml    # Entorno de producción
└── .env.example               # Plantilla de variables de entorno
```

## 2. Variables de entorno

### 2.1. Crear archivo `.env`

```bash
cp .env.example .env
```

### 2.2. Descripción de cada variable

| Variable | Obligatoria | Ejemplo | Descripción |
|----------|-------------|---------|-------------|
| `POSTGRES_USER` | Sí | `tramits` | Usuario de PostgreSQL |
| `POSTGRES_PASSWORD` | Sí | `cambiar123` | Contraseña de PostgreSQL |
| `POSTGRES_DB` | Sí | `tramits` | Nombre de la base de datos |
| `VITE_API_URL` | Sí | `http://localhost:3030` | URL del backend para el frontend |
| `DB_HOST` | Sí | `db` | Host de PostgreSQL (nombre del servicio Docker) |
| `DB_USER` | Sí | `tramits` | Usuario de BD (mismo que POSTGRES_USER) |
| `DB_PASSWORD` | Sí | `cambiar123` | Contraseña de BD |
| `DB_NAME` | Sí | `tramits` | Nombre de BD |
| `DB_PORT` | Sí | `5432` | Puerto de PostgreSQL |
| `ENVIRONMENT` | Sí | `dev` o `prod` | Entorno |
| `RUST_LOG` | No | `info` | Nivel de logging del backend |
| `PORT` | Sí | `3030` | Puerto del backend |
| `SPA_URL` | Sí | `http://localhost:5173` | Origen del frontend (CORS) |
| `JWT_SECRET` | Sí | `clave-secreta-muy-larga` | Secreto para firmar JWT |
| `ADMIN_EMAIL` | Sí | `admin@tramits.cu` | Email del admin inicial |
| `ADMIN_PASSWORD` | Sí | `admin123` | Contraseña del admin inicial |
| `CONSUMER_EMAIL` | Sí | `consumidor@seed.local` | Email del consumidor de prueba |
| `CONSUMER_PASSWORD` | Sí | `consumer123` | Contraseña del consumidor de prueba |
| `REGISTRAR_EMAIL` | Sí | `registrador@seed.local` | Email del registrador de prueba |
| `REGISTRAR_PASSWORD` | Sí | `registrar123` | Contraseña del registrador de prueba |
| `REDIS_URL` | Sí | `redis://cache:6379` | URL del servidor Redis |
| `MAIL_URL` | Sí | `smtp://mail:1025` | URL del servidor SMTP |
| `MAIL_USER` | Sí (si requiere auth) | — | Usuario SMTP |
| `MAIL_PASSWORD` | Sí (si requiere auth) | — | Contraseña SMTP |
| `TZ` | Sí | `America/Havana` | Zona horaria |

### 2.3. Notas sobre el correo SMTP

En desarrollo se incluye **MailHog** que captura todos los correos sin enviarlos realmente. Para ver los correos:

1. Inicia los servicios con docker compose
2. Abre http://localhost:8025 en el navegador

En producción reemplaza `MAIL_URL`, `MAIL_USER` y `MAIL_PASSWORD` con los datos de tu proveedor SMTP real (SendGrid, Mailgun, Gmail SMTP, etc.).

Ejemplo con Gmail SMTP:
```
MAIL_URL=smtp://smtp.gmail.com:587
MAIL_USER=tu-correo@gmail.com
MAIL_PASSWORD=tu-contrasena-de-aplicacion
```

---

## 3. Despliegue en desarrollo

### 3.1. Clonar el repositorio

```bash
git clone https://github.com/julioguirola/tramits.git
cd tramits
```

### 3.2. Configurar entorno

```bash
cp .env.example .env
# Editar .env con los valores adecuados
```

Ejemplo mínimo de `.env` para desarrollo:

```env
POSTGRES_USER=tramits
POSTGRES_PASSWORD=tramits123
POSTGRES_DB=tramits
VITE_API_URL=http://localhost:3030
DB_HOST=db
DB_USER=tramits
DB_PASSWORD=tramits123
DB_NAME=tramits
DB_PORT=5432
ENVIRONMENT=dev
RUST_LOG=info
PORT=3030
SPA_URL=http://localhost:5173
JWT_SECRET=dev-secret-key-change-in-prod
ADMIN_EMAIL=admin@tramits.cu
ADMIN_PASSWORD=admin123
CONSUMER_EMAIL=consumidor@seed.local
CONSUMER_PASSWORD=consumer123
REGISTRAR_EMAIL=registrador@seed.local
REGISTRAR_PASSWORD=registrar123
REDIS_URL=redis://cache:6379
MAIL_URL=smtp://mail:1025
MAIL_USER=
MAIL_PASSWORD=
TZ=America/Havana
```

### 3.3. Iniciar servicios

```bash
docker compose -f docker-compose.dev.yml --env-file .env up --build -d
```

Esto levanta:
- **frontend** → http://localhost:5173
- **backend** → http://localhost:3030
- **PostgreSQL** → puerto `5433` (mapeado desde 5432)
- **Redis** → puerto `6377`
- **MailHog** → SMTP `1025`, UI web en puerto `8025`

### 3.4. Inicializar la base de datos

El backend ejecuta migraciones y seed automáticamente al iniciar si la variable `MIGRATE=true` está configurada (por defecto se establece en el código del backend). Si necesitas ejecutarlo manualmente:

```bash
# Ejecutar dentro del contenedor del backend
docker exec -it tramits-backend-1 cargo run --bin migration
docker exec -it tramits-backend-1 cargo run --bin seeder
```

O si tienes Rust instalado localmente:

```bash
cd backend
cargo run --bin migration
cargo run --bin seeder
```

### 3.5. Usuarios de prueba (seed)

| Rol | Email | Contraseña |
|-----|-------|------------|
| Administrador | `admin@tramits.cu` | `admin123` |
| Consumidor | `consumidor@seed.local` | `consumer123` |
| Registrador | `registrador@seed.local` | `registrar123` |

### 3.6. Desarrollo con hot-reload

- **Frontend**: Se montan los directorios `src/`, `public/` y archivos de configuración como volúmenes bind. Cualquier cambio se refleja al instante.
- **Backend**: Se monta `backend/src/` como volumen. Al guardar un archivo, cargo detecta el cambio y recompila automáticamente (el contenedor ejecuta `cargo run` que incluye recompilación).

---

## 4. Despliegue en producción

### 4.1. Preparar el servidor

```bash
# Actualizar sistema
sudo apt update && sudo apt upgrade -y

# Instalar Docker
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER
# Cerrar sesión y volver a entrar para aplicar cambios

# Verificar instalación
docker --version
docker compose version
```

### 4.2. Obtener los archivos de despliegue

```bash
mkdir ~/tramits && cd ~/tramits

# Descargar el docker-compose de producción
curl -O https://raw.githubusercontent.com/julioguirola/tramits/master/docker-compose.prod.yml
```

### 4.3. Configurar variables de entorno

```bash
nano .env.prod
```

Ejemplo mínimo para producción:

```env
POSTGRES_USER=tramits
POSTGRES_PASSWORD=<generar-contraseña-segura>
POSTGRES_DB=tramits
VITE_API_URL=https://api.tudominio.com
DB_HOST=db
DB_USER=tramits
DB_PASSWORD=<generar-contraseña-segura>
DB_NAME=tramits
DB_PORT=5432
ENVIRONMENT=prod
RUST_LOG=warn
PORT=3030
SPA_URL=https://tudominio.com
JWT_SECRET=<generar-clave-segura-minimo-32-caracteres>
ADMIN_EMAIL=admin@tudominio.com
ADMIN_PASSWORD=<generar-contraseña-segura>
CONSUMER_EMAIL=consumidor@seed.local
CONSUMER_PASSWORD=consumer123
REGISTRAR_EMAIL=registrador@seed.local
REGISTRAR_PASSWORD=registrar123
REDIS_URL=redis://cache:6379
MAIL_URL=smtp://smtp.tuproveedor.com:587
MAIL_USER=correo@tudominio.com
MAIL_PASSWORD=<contraseña-smtp>
TZ=America/Havana
```

> **Seguridad**: Usa `openssl rand -base64 32` para generar `JWT_SECRET` y contraseñas robustas.

### 4.4. Iniciar servicios

```bash
docker compose -f docker-compose.prod.yml --env-file .env.prod up -d
```

### 4.5. Verificar que todo funciona

```bash
# Ver estado de los contenedores
docker compose -f docker-compose.prod.yml ps

# Ver logs
docker compose -f docker-compose.prod.yml logs -f

# Probar que el backend responde
curl http://localhost:3030
```

### 4.6. Configurar proxy inverso con Nginx (opcional pero recomendado)

Instalar Nginx:

```bash
sudo apt install nginx -y
```

Crear configuración en `/etc/nginx/sites-available/tramits`:

```nginx
server {
    listen 80;
    server_name tudominio.com;

    # Frontend SPA
    location / {
        proxy_pass http://localhost:80;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Backend API
    location /api/ {
        rewrite ^/api/(.*) /$1 break;
        proxy_pass http://localhost:3030;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # WebSocket si aplica
    # proxy_set_header Upgrade $http_upgrade;
    # proxy_set_header Connection "upgrade";
}
```

Habilitar y reiniciar:

```bash
sudo ln -s /etc/nginx/sites-available/tramits /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

### 4.7. TLS con Let's Encrypt (certbot)

```bash
sudo apt install certbot python3-certbot-nginx -y
sudo certbot --nginx -d tudominio.com
```

Esto configura automáticamente HTTPS y renueva los certificados cada 90 días.

---

## 5. Mantenimiento

### 5.1. Actualizar a nueva versión

```bash
cd ~/tramits

# Descargar nuevo docker-compose (si cambió)
curl -O https://raw.githubusercontent.com/julioguirola/tramits/master/docker-compose.prod.yml

# Bajar servicios
docker compose -f docker-compose.prod.yml down

# Descargar nuevas imágenes
docker compose -f docker-compose.prod.yml pull

# Levantar de nuevo
docker compose -f docker-compose.prod.yml up -d
```

### 5.2. Migraciones de base de datos

Las migraciones se ejecutan automáticamente al iniciar el backend. Si necesitas ejecutarlas manualmente en producción:

```bash
docker exec -it tramits-backend-1 cargo run --bin migration
```

### 5.3. Respaldos

```bash
# Respaldar la base de datos
docker exec -t tramits-db-1 pg_dump -U tramits tramits > backup_$(date +%Y%m%d).sql

# Restaurar
cat backup_20250101.sql | docker exec -i tramits-db-1 psql -U tramits tramits
```

### 5.4. Logs

```bash
# Ver logs de todos los servicios
docker compose -f docker-compose.prod.yml logs -f

# Ver logs de un servicio específico
docker compose -f docker-compose.prod.yml logs -f backend
docker compose -f docker-compose.prod.yml logs -f frontend
```

---

## 6. Solución de problemas

| Problema | Causa probable | Solución |
|----------|---------------|----------|
| Backend no inicia | Falta variable de entorno | Revisar `.env` contra `.env.example`. El backend hace panic si falta alguna variable |
| `connection refused` a DB | DB no lista aún | El backend reintenta automáticamente. Esperar 10 segundos |
| CORS bloquea peticiones | `SPA_URL` incorrecto | Verificar que coincida exactamente con la URL del frontend |
| No llegan correos | SMTP mal configurado | En dev usar MailHog (http://localhost:8025). En prod verificar credenciales SMTP |
| Seed duplicado | Ya se ejecutó el seeder | Las tablas se dropean y recrean en cada migration |
| Error de compilación Rust | Versión antigua | `rustup update` |
| Puerto ocupado | Otro servicio en el puerto | Cambiar `PORT` en `.env` |

---

## 7. Puertos por servicio

### Desarrollo

| Servicio | Puerto interno | Puerto anfitrión |
|----------|---------------|------------------|
| Frontend (Vite) | 5173 | 5173 |
| Backend | según `PORT` | según `PORT` |
| PostgreSQL | 5432 | 5433 |
| Redis | 6379 | 6377 |
| MailHog SMTP | 1025 | 1025 |
| MailHog UI | 8025 | 8025 |

### Producción

| Servicio | Puerto interno | Puerto anfitrión |
|----------|---------------|------------------|
| Frontend (Nginx) | 80 | 80 |
| Backend | 3030 | 3030 |
| PostgreSQL | 5432 | 5432 |
| Redis | 6379 | — (solo interno) |
| MailHog SMTP | 1025 | 1025 |
| MailHog UI | 8025 | 8025 |
