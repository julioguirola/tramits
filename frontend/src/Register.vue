<script lang="ts">
import SignupForm from "@/components/auth/SignupForm.vue";
export default {
  components: {
    SignupForm,
  },
};
</script>

<template>
  <div
    class="min-h-[100svh] overflow-y-auto bg-[#f7fbf8] text-slate-900 auth-font-body pt-[env(safe-area-inset-top)] pb-[env(safe-area-inset-bottom)]"
  >
    <div class="relative overflow-x-hidden">
      <div
        class="pointer-events-none absolute -left-20 top-12 h-72 w-72 rounded-full bg-[radial-gradient(circle_at_center,rgba(16,185,129,0.3),rgba(16,185,129,0))] blur-3xl"
        aria-hidden="true"
      ></div>
      <div
        class="pointer-events-none absolute -right-24 top-0 h-80 w-80 rounded-full bg-[radial-gradient(circle_at_center,rgba(20,184,166,0.25),rgba(20,184,166,0))] blur-3xl"
        aria-hidden="true"
      ></div>
      <div
        class="relative mx-auto grid max-w-6xl gap-10 px-6 py-12 sm:py-14 lg:min-h-[100svh] lg:grid-cols-[0.9fr_1.1fr] lg:items-center"
      >
        <section class="space-y-8">
          <div class="inline-flex items-center gap-2 rounded-full bg-white/70 px-4 py-2 text-sm font-semibold text-emerald-800 shadow-sm ring-1 ring-emerald-100">
            <span class="h-2 w-2 rounded-full bg-emerald-500"></span>
            Bienvenido a Tramits
          </div>
          <div class="space-y-4">
            <h1
              class="auth-font-display text-4xl font-semibold leading-tight text-slate-900 sm:text-5xl"
            >
              Crea tu cuenta y comienza tus trámites hoy mismo.
            </h1>
            <p class="max-w-xl text-base text-slate-600 sm:text-lg">
              Valida tu identidad con el carné, completa tus datos y recibe
              actualizaciones en tiempo real desde cualquier dispositivo.
            </p>
          </div>
          <div
            class="rounded-3xl border border-emerald-100 bg-white/60 p-4 shadow-sm"
          >
            <img
              src="/tramites.webp"
              alt="Ilustración de trámites"
              class="h-56 w-full rounded-2xl object-cover"
            />
          </div>
          <div class="grid gap-4 sm:grid-cols-2">
            <div class="rounded-2xl bg-white/80 p-5 shadow-sm ring-1 ring-emerald-100">
              <h3 class="text-base font-semibold text-slate-900">
                Verificación rápida
              </h3>
              <p class="mt-2 text-sm text-slate-600">
                Confirmamos tu identidad en segundos para evitar errores en el
                registro.
              </p>
            </div>
            <div class="rounded-2xl bg-white/80 p-5 shadow-sm ring-1 ring-emerald-100">
              <h3 class="text-base font-semibold text-slate-900">
                Soporte disponible
              </h3>
              <p class="mt-2 text-sm text-slate-600">
                Resolvemos tus dudas mientras completas el formulario.
              </p>
            </div>
          </div>
        </section>
        <section class="space-y-6">
          <div class="auth-panel">
            <SignupForm />
          </div>
          <div class="grid gap-4 sm:grid-cols-2">
            <div class="rounded-2xl bg-white/80 p-5 shadow-sm ring-1 ring-emerald-100">
              <p class="text-xs uppercase tracking-wide text-emerald-700">
                Transparencia
              </p>
              <p class="mt-2 text-sm text-slate-600">
                Visualiza cada etapa del proceso desde tu panel.
              </p>
            </div>
            <div class="rounded-2xl bg-white/80 p-5 shadow-sm ring-1 ring-emerald-100">
              <p class="text-xs uppercase tracking-wide text-emerald-700">
                Notificaciones
              </p>
              <p class="mt-2 text-sm text-slate-600">
                Recibe alertas al instante sobre cambios de estado.
              </p>
            </div>
          </div>
        </section>
      </div>
    </div>
  </div>
</template>
