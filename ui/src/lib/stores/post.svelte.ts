import type {
  Config,
  Channel,
  ImageItem,
  ChannelPostResult,
  TranslationResult,
} from "../types";

class PostStore {
  text = $state("");
  images = $state<ImageItem[]>([]);
  selectedChannels = $state<string[]>([]);
  scheduleEnabled = $state(false);
  scheduleDatetime = $state("");
  config = $state<Config | null>(null);
  translations = $state<TranslationResult[]>([]);
  results = $state<ChannelPostResult[]>([]);
  isPosting = $state(false);
  isTranslating = $state(false);
  error = $state<string | null>(null);

  get channels(): Channel[] {
    return this.config?.channels ?? [];
  }

  get schedule(): string | null {
    if (!this.scheduleEnabled || !this.scheduleDatetime) return null;
    return new Date(this.scheduleDatetime).toISOString();
  }

  get imagePaths(): string[] {
    return this.images.map((img) => img.path);
  }

  get canPost(): boolean {
    return (
      this.text.trim().length > 0 &&
      this.selectedChannels.length > 0 &&
      !this.isPosting
    );
  }

  get translatingChannels(): Channel[] {
    return this.channels.filter(
      (c) =>
        c.should_translate &&
        c.translate &&
        this.selectedChannels.includes(c.name)
    );
  }

  toggleChannel(name: string) {
    if (this.selectedChannels.includes(name)) {
      this.selectedChannels = this.selectedChannels.filter((n) => n !== name);
    } else {
      this.selectedChannels = [...this.selectedChannels, name];
    }
  }

  addImage(item: ImageItem) {
    this.images = [...this.images, item];
  }

  removeImage(index: number) {
    this.images = this.images.filter((_, i) => i !== index);
  }

  updateTranslation(channel: string, text: string) {
    this.translations = this.translations.map((t) =>
      t.channel === channel ? { ...t, translated_text: text } : t
    );
  }

  reset() {
    this.text = "";
    this.images = [];
    this.scheduleEnabled = false;
    this.scheduleDatetime = "";
    this.translations = [];
    this.results = [];
    this.error = null;
  }

  newPost() {
    this.reset();
    this.selectedChannels = [...(this.config?.default_post_channels ?? [])];
  }
}

export const postStore = new PostStore();
