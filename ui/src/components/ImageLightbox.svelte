<script lang="ts">
  import { lightbox } from "../lib/stores/lightbox.svelte";

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      lightbox.close();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if lightbox.src}
  <div class="lightbox-backdrop" onclick={() => lightbox.close()} role="dialog">
    <img src={lightbox.src} alt="Full preview" class="lightbox-img" />
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
</style>
