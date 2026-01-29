<script lang="ts">
  interface Props {
    direction: "horizontal" | "vertical";
    side: "left" | "right" | "top" | "bottom";
    onresize: (delta: number) => void;
  }

  let { direction, side, onresize }: Props = $props();

  let isDragging = $state(false);

  function handleMouseDown(e: MouseEvent) {
    isDragging = true;
    window.addEventListener("mousemove", handleMouseMove);
    window.addEventListener("mouseup", handleMouseUp);
    document.body.style.cursor =
      direction === "horizontal" ? "col-resize" : "row-resize";
    document.body.style.userSelect = "none";
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isDragging) return;

    if (direction === "horizontal") {
      onresize(e.movementX);
    } else {
      onresize(e.movementY);
    }
  }

  function handleMouseUp() {
    isDragging = false;
    window.removeEventListener("mousemove", handleMouseMove);
    window.removeEventListener("mouseup", handleMouseUp);
    document.body.style.cursor = "";
    document.body.style.userSelect = "";
  }
</script>

<div
  class="resize-handle {direction} {side} {isDragging ? 'dragging' : ''}"
  onmousedown={handleMouseDown}
  role="presentation"
></div>

<style>
  .resize-handle {
    position: absolute;
    z-index: 100;
    transition: background-color 0.1s;
    background-color: transparent;
  }

  .resize-handle.horizontal {
    width: 6px;
    height: 100%;
    cursor: col-resize;
  }

  .resize-handle.vertical {
    height: 6px;
    width: 100%;
    cursor: row-resize;
  }

  .resize-handle.right {
    right: -2px;
    top: 0;
  }

  .resize-handle.left {
    left: -2px;
    top: 0;
  }

  .resize-handle.top {
    top: -2px;
    left: 0;
  }

  .resize-handle.bottom {
    bottom: -2px;
    left: 0;
  }

  .resize-handle:hover,
  .resize-handle.dragging {
    background-color: transparent;
  }

  /* Thin visual line inside the thicker hit area */
  .resize-handle::after {
    content: "";
    position: absolute;
    background-color: transparent;
    transition: background-color 0.1s;
  }

  .resize-handle.horizontal::after {
    width: 1px;
    height: 100%;
    left: 50%;
    transform: translateX(-50%);
  }

  .resize-handle.vertical::after {
    height: 1px;
    width: 100%;
    top: 50%;
    transform: translateY(-50%);
  }

  .resize-handle:hover::after,
  .resize-handle.dragging::after {
    background-color: var(--border-subtle);
  }
</style>
