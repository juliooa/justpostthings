import { invoke } from "@tauri-apps/api/core";
import type {
  Config,
  Settings,
  ChannelPostResult,
  TranslationResult,
  Idea,
} from "./types";

export async function getSettings(): Promise<Settings> {
  return invoke<Settings>("get_settings");
}

export async function saveSettings(settings: Settings): Promise<void> {
  return invoke<void>("save_settings", { settings });
}

export async function getConfig(): Promise<Config> {
  return invoke<Config>("get_config");
}

export async function translatePreview(
  text: string,
  channelNames: string[]
): Promise<TranslationResult[]> {
  return invoke<TranslationResult[]>("translate_preview", {
    text,
    channelNames,
  });
}

export async function uploadImage(filePath: string): Promise<string> {
  return invoke<string>("upload_image", { filePath });
}

export async function submitPost(
  text: string,
  images: string[],
  schedule: string | null,
  channelNames: string[],
  textOverrides: Record<string, string> = {},
  scheduleOverrides: Record<string, string> = {}
): Promise<ChannelPostResult[]> {
  return invoke<ChannelPostResult[]>("submit_post", {
    text,
    images,
    schedule,
    channelNames,
    textOverrides,
    scheduleOverrides,
  });
}

export async function saveSentPost(
  channelTexts: [string, string][]
): Promise<void> {
  return invoke<void>("save_sent_post", { channelTexts });
}

export async function openPostsFolder(): Promise<void> {
  return invoke<void>("open_posts_folder");
}

export async function readImageBase64(filePath: string): Promise<string> {
  return invoke<string>("read_image_base64", { filePath });
}

export async function shrinkText(
  text: string,
  maxChars?: number
): Promise<string> {
  return invoke<string>("shrink_text", { text, maxChars });
}

export async function listIdeas(): Promise<Idea[]> {
  return invoke<Idea[]>("list_ideas");
}

export async function createIdea(content: string): Promise<Idea> {
  return invoke<Idea>("create_idea", { content });
}

export async function updateIdea(id: string, content: string): Promise<void> {
  return invoke<void>("update_idea", { id, content });
}

export async function deleteIdea(id: string): Promise<void> {
  return invoke<void>("delete_idea", { id });
}
