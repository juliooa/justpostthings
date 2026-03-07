export interface TranslateConfig {
  from: string;
  to: string;
}

export interface Channel {
  name: string;
  id: string;
  should_translate: boolean;
  translate?: TranslateConfig;
}

export interface Config {
  channels: Channel[];
  default_post_channels: string[];
  translation_service?: string;
}

export interface ChannelPostResult {
  channel: string;
  success: boolean;
  message: string;
  post_id?: string;
}

export interface TranslationResult {
  channel: string;
  translated_text: string;
}

export interface ImageItem {
  path: string;
  preview: string; // data URL for local, or the URL itself
  isLocal: boolean;
}
