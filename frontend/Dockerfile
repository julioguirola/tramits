FROM oven/bun:latest AS builder

WORKDIR /app

ARG VITE_API_URL
ENV VITE_API_URL=$VITE_API_URL

COPY ./package.json .
COPY ./bun.lock .

RUN bun i

COPY . .

RUN bun run build-only

FROM nginx:trixie-perl

COPY default.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]