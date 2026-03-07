import { invoke } from "@tauri-apps/api/core";
import type {
  Config,
  ChannelPostResult,
  TranslationResult,
} from "./types";

export async function getConfig(): Promise<Config> {
  return invoke<Config>("get_config", { configPath: null });
}

export async function translatePreview(
  text: string,
  channelNames: string[]
): Promise<TranslationResult[]> {
  return invoke<TranslationResult[]>("translate_preview", {
    text,
    channelNames,
    configPath: null,
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
  textOverrides: Record<string, string> = {}
): Promise<ChannelPostResult[]> {
  return invoke<ChannelPostResult[]>("submit_post", {
    text,
    images,
    schedule,
    channelNames,
    textOverrides,
    configPath: null,
  });
}

export async function readImageBase64(filePath: string): Promise<string> {
  return invoke<string>("read_image_base64", { filePath });
}
