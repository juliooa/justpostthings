import type { Settings, Config, Prompts } from "../types";
import { getSettings, saveSettings } from "../api";

class SettingsStore {
  channels = $state<Config["channels"]>([]);
  default_post_channels = $state<string[]>([]);
  llm_service = $state<Config["llm_service"]>(undefined);
  save_sent_posts = $state(true);
  prompts = $state<Prompts | undefined>(undefined);
  bufferApiKey = $state("");
  openaiApiKey = $state("");
  geminiApiKey = $state("");
  imgbbApiKey = $state("");
  loaded = $state(false);
  hasSettings = $state(false);

  get config(): Config {
    return {
      channels: this.channels,
      default_post_channels: this.default_post_channels,
      llm_service: this.llm_service,
      save_sent_posts: this.save_sent_posts,
      prompts: this.prompts,
    };
  }

  async load(): Promise<boolean> {
    try {
      const s = await getSettings();
      this.channels = s.channels;
      this.default_post_channels = s.default_post_channels;
      this.llm_service = s.llm_service;
      this.save_sent_posts = s.save_sent_posts ?? true;
      this.prompts = s.prompts;
      this.bufferApiKey = s.buffer_api_key;
      this.openaiApiKey = s.openai_api_key;
      this.geminiApiKey = s.gemini_api_key;
      this.imgbbApiKey = s.imgbb_api_key;
      this.loaded = true;
      this.hasSettings = true;
      return true;
    } catch {
      this.loaded = true;
      this.hasSettings = false;
      return false;
    }
  }

  async save(): Promise<void> {
    const settings: Settings = {
      channels: this.channels,
      default_post_channels: this.default_post_channels,
      llm_service: this.llm_service,
      save_sent_posts: this.save_sent_posts,
      prompts: this.prompts,
      buffer_api_key: this.bufferApiKey,
      openai_api_key: this.openaiApiKey,
      gemini_api_key: this.geminiApiKey,
      imgbb_api_key: this.imgbbApiKey,
    };
    await saveSettings(settings);
    this.hasSettings = true;
  }
}

export const settingsStore = new SettingsStore();
