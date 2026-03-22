<script lang="ts">
  import { lightbox } from "../lib/stores/lightbox.svelte";
  import { postStore } from "../lib/stores/post.svelte";
  import { saveDrawingImage, readImageBase64 } from "../lib/api";

  let drawingMode = $state(false);
  let canvas: HTMLCanvasElement;
  let isDrawing = $state(false);
  let selectedColor = $state("#ff3b30");
  let isSaving = $state(false);
  let lastX = 0;
  let lastY = 0;

  const colors = [
    { value: "#ff3b30", label: "Red" },
    { value: "#ffffff", label: "White" },
    { value: "#007aff", label: "Blue" },
  ];

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      if (drawingMode) {
        exitDrawingMode();
      } else {
        lightbox.close();
      }
    }
  }

  function enterDrawingMode() {
    drawingMode = true;
    requestAnimationFrame(() => {
      if (!canvas) return;
      const ctx = canvas.getContext("2d")!;
      const img = new Image();
      img.crossOrigin = "anonymous";
      img.onload = () => {
        canvas.width = img.naturalWidth;
        canvas.height = img.naturalHeight;
        ctx.drawImage(img, 0, 0);
      };
      img.src = lightbox.src!;
    });
  }

  function exitDrawingMode() {
    drawingMode = false;
  }

  function getScale() {
    const rect = canvas.getBoundingClientRect();
    return {
      x: canvas.width / rect.width,
      y: canvas.height / rect.height,
    };
  }

  function startDraw(e: PointerEvent) {
    isDrawing = true;
    canvas.setPointerCapture(e.pointerId);
    const rect = canvas.getBoundingClientRect();
    const scale = getScale();
    lastX = (e.clientX - rect.left) * scale.x;
    lastY = (e.clientY - rect.top) * scale.y;
  }

  function draw(e: PointerEvent) {
    if (!isDrawing) return;
    const ctx = canvas.getContext("2d")!;
    const rect = canvas.getBoundingClientRect();
    const scale = getScale();
    const x = (e.clientX - rect.left) * scale.x;
    const y = (e.clientY - rect.top) * scale.y;

    ctx.beginPath();
    ctx.moveTo(lastX, lastY);
    ctx.lineTo(x, y);
    ctx.strokeStyle = selectedColor;
    ctx.lineWidth = Math.max(canvas.width, canvas.height) / 250;
    ctx.lineCap = "round";
    ctx.lineJoin = "round";
    ctx.stroke();

    lastX = x;
    lastY = y;
  }

  function endDraw() {
    isDrawing = false;
  }

  async function saveDrawing() {
    if (!canvas || lightbox.imageIndex === null) return;
    isSaving = true;
    try {
      const dataUrl = canvas.toDataURL("image/png");
      const base64 = dataUrl.split(",")[1];
      const tempPath = await saveDrawingImage(base64);
      const preview = await readImageBase64(tempPath);

      postStore.replaceImage(lightbox.imageIndex, {
        path: tempPath,
        preview,
        isLocal: true,
      });
      postStore.addTempPath(tempPath);

      drawingMode = false;
      lightbox.close();
    } catch (e) {
      postStore.error = `Failed to save drawing: ${e}`;
    } finally {
      isSaving = false;
    }
  }

  function handleBackdropClick(e: MouseEvent) {
    if ((e.target as HTMLElement).classList.contains("lightbox-backdrop")) {
      if (drawingMode) {
        exitDrawingMode();
      } else {
        lightbox.close();
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if lightbox.src}
  <div
    class="lightbox-backdrop"
    onclick={handleBackdropClick}
    role="dialog"
  >
    {#if drawingMode}
      <div class="drawing-container">
        <div class="drawing-toolbar">
          <div class="color-picker">
            {#each colors as color}
              <button
                class="color-btn"
                class:active={selectedColor === color.value}
                style="background: {color.value};"
                title={color.label}
                onclick={() => (selectedColor = color.value)}
              ></button>
            {/each}
          </div>
          <div class="drawing-actions">
            <button class="toolbar-btn cancel" onclick={exitDrawingMode}>
              Cancel
            </button>
            <button
              class="toolbar-btn save"
              onclick={saveDrawing}
              disabled={isSaving}
            >
              {isSaving ? "Saving..." : "Save"}
            </button>
          </div>
        </div>
        <canvas
          bind:this={canvas}
          class="drawing-canvas"
          onpointerdown={startDraw}
          onpointermove={draw}
          onpointerup={endDraw}
          onpointerleave={endDraw}
        ></canvas>
      </div>
    {:else}
      <img src={lightbox.src} alt="Full preview" class="lightbox-img" />
      {#if lightbox.imageIndex !== null}
        <button class="edit-btn" title="Draw on image" onclick={enterDrawingMode}>
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"></path>
          </svg>
        </button>
      {/if}
    {/if}
  </div>
{/if}

<style>
  .lightbox-backdrop {
    position: fixed;
    inset: 0;
    z-index: 9999;
    background: rgba(0, 0, 0, 0.85);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: zoom-out;
    animation: fade-in 0.15s ease;
  }

  @keyframes fade-in {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .lightbox-img {
    max-width: 90vw;
    max-height: 90vh;
    object-fit: contain;
    border-radius: 8px;
    box-shadow: 0 8px 40px rgba(0, 0, 0, 0.6);
    animation: scale-in 0.15s ease;
  }

  @keyframes scale-in {
    from { transform: scale(0.95); opacity: 0; }
    to { transform: scale(1); opacity: 1; }
  }

  .edit-btn {
    position: fixed;
    bottom: 32px;
    right: 32px;
    width: 44px;
    height: 44px;
    border-radius: 50%;
    border: 1px solid rgba(255, 255, 255, 0.2);
    background: rgba(255, 255, 255, 0.1);
    color: white;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s ease;
    backdrop-filter: blur(10px);
  }

  .edit-btn:hover {
    background: rgba(255, 255, 255, 0.2);
    border-color: rgba(255, 255, 255, 0.4);
    transform: scale(1.1);
  }

  .drawing-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    cursor: default;
  }

  .drawing-toolbar {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 8px 16px;
    background: rgba(30, 30, 30, 0.9);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    backdrop-filter: blur(10px);
  }

  .color-picker {
    display: flex;
    gap: 8px;
  }

  .color-btn {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    border: 2px solid rgba(255, 255, 255, 0.2);
    cursor: pointer;
    transition: all 0.15s ease;
    padding: 0;
  }

  .color-btn.active {
    border-color: white;
    transform: scale(1.15);
    box-shadow: 0 0 8px rgba(255, 255, 255, 0.3);
  }

  .color-btn:hover:not(.active) {
    border-color: rgba(255, 255, 255, 0.5);
    transform: scale(1.05);
  }

  .drawing-actions {
    display: flex;
    gap: 8px;
    margin-left: 8px;
  }

  .toolbar-btn {
    padding: 6px 16px;
    border-radius: 8px;
    border: none;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    font-family: "DM Sans", system-ui, sans-serif;
    transition: all 0.15s ease;
  }

  .toolbar-btn.cancel {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.7);
  }

  .toolbar-btn.cancel:hover {
    background: rgba(255, 255, 255, 0.15);
    color: white;
  }

  .toolbar-btn.save {
    background: var(--accent, #7c5cfc);
    color: white;
  }

  .toolbar-btn.save:hover:not(:disabled) {
    filter: brightness(1.15);
  }

  .toolbar-btn.save:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .drawing-canvas {
    max-width: 90vw;
    max-height: calc(90vh - 60px);
    object-fit: contain;
    border-radius: 8px;
    cursor: crosshair;
    touch-action: none;
    box-shadow: 0 8px 40px rgba(0, 0, 0, 0.6);
  }
</style>
