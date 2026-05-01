<script lang="ts">
import LoginForm from "@/components/auth/LoginForm.vue";
export default {
  data() {
    return {};
  },
  components: {
    LoginForm,
  },
};
</script>

<template>
  <div class="min-h-[100svh] overflow-y-auto bg-[#f6fbf7] text-slate-900 auth-font-body pt-[env(safe-area-inset-top)] pb-[env(safe-area-inset-bottom)]">
    <div class="relative overflow-x-hidden">
      <div
        class="pointer-events-none absolute -left-24 top-0 h-72 w-72 rounded-full bg-[radial-gradient(circle_at_center,rgba(16,185,129,0.35),rgba(16,185,129,0))] blur-3xl"
        aria-hidden="true"
      ></div>
      <div
        class="pointer-events-none absolute -right-16 top-24 h-80 w-80 rounded-full bg-[radial-gradient(circle_at_center,rgba(14,116,144,0.25),rgba(14,116,144,0))] blur-3xl"
        aria-hidden="true"
      ></div>
      <div
        class="pointer-events-none absolute bottom-0 left-1/3 h-64 w-64 rounded-full bg-[radial-gradient(circle_at_center,rgba(34,197,94,0.18),rgba(34,197,94,0))] blur-3xl"
        aria-hidden="true"
      ></div>
      <div
        class="relative mx-auto grid max-w-6xl gap-10 px-6 py-12 sm:py-14 lg:min-h-[100svh] lg:grid-cols-[1.2fr_0.8fr] lg:items-center"
      >
        <section class="space-y-8">
          <div class="inline-flex items-center gap-2 rounded-full bg-white/70 px-4 py-2 text-sm font-semibold text-emerald-800 shadow-sm ring-1 ring-emerald-100">
            <span class="h-2 w-2 rounded-full bg-emerald-500"></span>
            Tramits Digital
          </div>
          <div class="space-y-4">
            <h1 class="auth-font-display text-4xl font-semibold leading-tight text-slate-900 sm:text-5xl">
              Gestiona tus tramites sin filas, con claridad y control.
            </h1>
            <p class="max-w-xl text-base text-slate-600 sm:text-lg">
              Centraliza solicitudes, acompanamiento y aprobaciones desde un
              unico lugar. Disenado para oficinas y ciudadanos que necesitan
              respuestas rapidas.
            </p>
          </div>
          <div class="grid gap-4 sm:grid-cols-3">
            <div class="rounded-2xl bg-white/80 p-4 shadow-sm ring-1 ring-emerald-100">
              <p class="text-xs uppercase tracking-wide text-emerald-700">Cobertura</p>
              <p class="text-2xl font-semibold text-slate-900">15 provincias</p>
            </div>
            <div class="rounded-2xl bg-white/80 p-4 shadow-sm ring-1 ring-emerald-100">
              <p class="text-xs uppercase tracking-wide text-emerald-700">Tiempo medio</p>
              <p class="text-2xl font-semibold text-slate-900">24 h</p>
            </div>
            <div class="rounded-2xl bg-white/80 p-4 shadow-sm ring-1 ring-emerald-100">
              <p class="text-xs uppercase tracking-wide text-emerald-700">Seguimiento</p>
              <p class="text-2xl font-semibold text-slate-900">En tiempo real</p>
            </div>
          </div>
          <div class="grid gap-4 sm:grid-cols-2">
            <div class="rounded-2xl border border-emerald-100 bg-white/70 p-5">
              <h3 class="text-base font-semibold text-slate-900">Proceso guiado</h3>
              <p class="mt-2 text-sm text-slate-600">
                Te acompanamos con pasos claros, alertas y validaciones para que
                no falte ningun documento.
              </p>
            </div>
            <div class="rounded-2xl border border-emerald-100 bg-white/70 p-5">
              <h3 class="text-base font-semibold text-slate-900">Historial ordenado</h3>
              <p class="mt-2 text-sm text-slate-600">
                Consulta el avance de cada tramite y guarda evidencia en un solo
                panel de control.
              </p>
            </div>
          </div>
          <div class="rounded-3xl border border-emerald-100 bg-white/60 p-4 shadow-sm">
            <img
              src="/tramites.webp"
              alt="Ilustracion de tramites"
              class="h-56 w-full rounded-2xl object-cover"
            />
          </div>
        </section>
        <section class="space-y-6">
          <div class="auth-panel">
            <LoginForm />
          </div>
          <div class="grid gap-4 sm:grid-cols-2">
            <div class="rounded-2xl bg-white/80 p-5 shadow-sm ring-1 ring-emerald-100">
              <p class="text-xs uppercase tracking-wide text-emerald-700">Seguridad</p>
              <p class="mt-2 text-sm text-slate-600">
                Acceso cifrado y control de roles para cada oficina.
              </p>
            </div>
            <div class="rounded-2xl bg-white/80 p-5 shadow-sm ring-1 ring-emerald-100">
              <p class="text-xs uppercase tracking-wide text-emerald-700">Soporte</p>
              <p class="mt-2 text-sm text-slate-600">
                Equipo tecnico disponible para ayudarte con cualquier incidente.
              </p>
            </div>
          </div>
        </section>
      </div>
    </div>
  </div>
</template>
