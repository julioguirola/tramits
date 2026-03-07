import { defineStore } from "pinia";

export const useJwtStore = defineStore("jwt", {
  state: (): { jwt: string } => ({ jwt: "" }),
  actions: {
    setJwt(val: string) {
      this.jwt = val;
    },
  },
});
