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

export interface LlmService {
  provider: string;
  model?: string;
}

export interface Prompts {
  translation_prompt?: string;
  shrink_prompt?: string;
}

export interface Config {
  channels: Channel[];
  default_post_channels: string[];
  llm_service?: LlmService;
  save_sent_posts?: boolean;
  prompts?: Prompts;
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

export interface Settings extends Config {
  buffer_api_key: string;
  openai_api_key: string;
  gemini_api_key: string;
  imgbb_api_key: string;
}

export interface Idea {
  id: string;
  content: string;
}

export interface SentPost {
  id: string;
  content: string;
}
