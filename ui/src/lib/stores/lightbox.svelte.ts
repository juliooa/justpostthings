class LightboxStore {
  src = $state<string | null>(null);

  open(src: string) {
    this.src = src;
  }

  close() {
    this.src = null;
  }
}

export const lightbox = new LightboxStore();
