import type { ClassValue } from "clsx";
import { clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import axios from "axios";
import { toast } from "vue-sonner";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
});

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
      toast.success(Respuesta.Error, { description: res.description });
      break;
    case Respuesta.Warn:
      toast.success(Respuesta.Warn, { description: res.description });
      break;
    case Respuesta.Info:
      toast.success(Respuesta.Info, { description: res.description });
      break;
    default:
      toast.error(Respuesta.Error, { description: res });
  }
}
