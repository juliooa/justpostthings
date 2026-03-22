class LightboxStore {
  src = $state<string | null>(null);
  imageIndex = $state<number | null>(null);

  open(src: string, index?: number) {
    this.src = src;
    this.imageIndex = index ?? null;
  }

  close() {
    this.src = null;
    this.imageIndex = null;
  }
}

export const lightbox = new LightboxStore();
