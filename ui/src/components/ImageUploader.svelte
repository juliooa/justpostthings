<script lang="ts">
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { postStore } from "../lib/stores/post.svelte";
  import { readImageBase64 } from "../lib/api";

  let isUploading = $state(false);
  let isDragging = $state(false);

  const imageExts = [".png", ".jpg", ".jpeg", ".gif", ".webp"];

  function isImagePath(path: string): boolean {
    const lower = path.toLowerCase();
    return imageExts.some((ext) => lower.endsWith(ext));
  }

  async function loadFiles(paths: string[]) {
    isUploading = true;
    try {
      for (const filePath of paths) {
        const preview = await readImageBase64(filePath);
        postStore.addImage({ path: filePath, preview, isLocal: true });
      }
    } catch (e) {
      postStore.error = `Failed to load image: ${e}`;
    } finally {
      isUploading = false;
    }
  }

  onMount(() => {
    const appWindow = getCurrentWebviewWindow();
    let unlisten: (() => void) | undefined;

    appWindow.onDragDropEvent((event) => {
      if (event.payload.type === "over") {
        isDragging = true;
      } else if (event.payload.type === "drop") {
        isDragging = false;
        const paths = event.payload.paths.filter(isImagePath);
        if (paths.length > 0) {
          loadFiles(paths);
        }
      } else if (event.payload.type === "leave") {
        isDragging = false;
      }
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten?.();
    };
  });

  async function pickImages() {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Images",
          extensions: ["png", "jpg", "jpeg", "gif", "webp"],
        },
      ],
    });

    if (!selected) return;
    const paths = Array.isArray(selected) ? selected : [selected];
    await loadFiles(paths);
  }

  function addUrl() {
    const url = prompt("Enter image URL:");
    if (url && (url.startsWith("http://") || url.startsWith("https://"))) {
      postStore.addImage({ path: url, preview: url, isLocal: false });
    }
  }
</script>

<div class="image-uploader">
  <span class="section-label">Media</span>

  <div
    class="drop-zone"
    class:dragging={isDragging}
    class:has-images={postStore.images.length > 0}
    role="region"
  >
    {#if postStore.images.length > 0}
      <div class="thumbnail-grid">
        {#each postStore.images as image, i}
          <div class="thumbnail">
            <img src={image.preview} alt="Upload {i + 1}" />
            <div class="thumbnail-overlay">
              <button class="remove-btn" onclick={() => postStore.removeImage(i)}>&times;</button>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="drop-prompt">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><circle cx="8.5" cy="8.5" r="1.5"/><polyline points="21 15 16 10 5 21"/>
        </svg>
        <span>Drop images here</span>
      </div>
    {/if}
  </div>

  <div class="actions">
    <button class="btn-secondary" onclick={pickImages} disabled={isUploading}>
      {#if isUploading}
        <span class="upload-spinner"></span>
      {/if}
      {isUploading ? "Loading..." : "Browse files"}
    </button>
    <button class="btn-secondary" onclick={addUrl}>Add URL</button>
  </div>
</div>

<style>
  .image-uploader {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .drop-zone {
    border: 1.5px dashed var(--border-strong);
    border-radius: var(--radius-lg);
    padding: 16px;
    transition: all 0.2s ease;
    min-height: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .drop-zone.has-images {
    padding: 10px;
    justify-content: flex-start;
  }

  .drop-zone.dragging {
    border-color: var(--accent);
    background: var(--accent-subtle);
    box-shadow: inset 0 0 20px var(--accent-glow);
  }

  .drop-prompt {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    color: var(--text-tertiary);
    font-size: 12.5px;
    user-select: none;
  }

  .dragging .drop-prompt {
    color: var(--accent);
  }

  .actions {
    display: flex;
    gap: 8px;
  }

  .upload-spinner {
    display: inline-block;
    width: 10px;
    height: 10px;
    border: 1.5px solid var(--border);
    border-top-color: var(--text-secondary);
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
    margin-right: 4px;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .thumbnail-grid {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .thumbnail {
    position: relative;
    width: 76px;
    height: 76px;
    border-radius: var(--radius);
    overflow: hidden;
    border: 1px solid var(--border);
    cursor: default;
  }

  .thumbnail img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.2s ease;
  }

  .thumbnail-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0);
    display: flex;
    align-items: flex-start;
    justify-content: flex-end;
    padding: 4px;
    transition: background 0.15s ease;
  }

  .thumbnail:hover .thumbnail-overlay {
    background: rgba(0, 0, 0, 0.4);
  }

  .thumbnail:hover img {
    transform: scale(1.05);
  }

  .remove-btn {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    border: none;
    background: rgba(0, 0, 0, 0.8);
    color: white;
    font-size: 14px;
    line-height: 1;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    opacity: 0;
    transform: scale(0.8);
    transition: all 0.15s ease;
  }

  .thumbnail:hover .remove-btn {
    opacity: 1;
    transform: scale(1);
  }

  .remove-btn:hover {
    background: var(--danger);
  }
</style>
