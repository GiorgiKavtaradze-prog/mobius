const dataElement = document.getElementById("demo-data");
const demos = dataElement ? JSON.parse(dataElement.textContent || "[]") : [];

const primaryVideo = document.querySelector(".demo-video-primary");
const secondaryVideo = document.querySelector(".demo-video-secondary");
const videoStack = document.querySelector(".video-stack");
const titleElement = document.querySelector(".demo-title");
const descriptionElement = document.querySelector(".demo-description");
const copyElement = document.querySelector(".demo-copy");
const leftButton = document.querySelector(".slider-button-left");
const rightButton = document.querySelector(".slider-button-right");
const loadingScreen = document.querySelector(".loading-screen");
const SLIDE_TRANSITION_MS = 280;
const VIDEO_LOADING_FADE_MS = 380;
const MIN_VIDEO_LOADING_VISIBLE_MS = window.matchMedia("(max-width: 800px)")
  .matches
  ? 900
  : 420;

let currentIndex = 0;
let isAnimating = false;
let showingPrimary = true;
let frameLocked = false;
let hasInitializedDemo = false;
let videoLoadingStartedAt = 0;

function lockFrameSize() {
  if (!(primaryVideo instanceof HTMLVideoElement) || !videoStack || frameLocked) {
    return;
  }

  const rect = primaryVideo.getBoundingClientRect();
  if (rect.width > 0 && rect.height > 0) {
    videoStack.style.width = `${Math.round(rect.width)}px`;
    videoStack.style.height = `${Math.round(rect.height)}px`;
    frameLocked = true;
  }
}

function showVideoLoading() {
  videoLoadingStartedAt = performance.now();
  videoStack?.classList.remove("is-loaded");
}

function hideVideoLoading() {
  videoStack?.classList.add("is-loaded");
}

function setCopyText(title, description) {
  if (titleElement) {
    titleElement.textContent = title;
  }

  if (descriptionElement) {
    descriptionElement.textContent = description;
  }
}

function applyDemo(videoElement, demo, showLoader = true) {
  if (!(videoElement instanceof HTMLVideoElement) || !demo) {
    return;
  }

  videoElement.classList.remove("is-loading");
  copyElement?.classList.remove("is-loading");

  if (showLoader) {
    showVideoLoading();
    videoElement.classList.add("is-loading");
    copyElement?.classList.add("is-loading");
    setCopyText(demo.title, demo.description);
  } else {
    hideVideoLoading();
  }
  videoElement.src = demo.src;
  videoElement.setAttribute("aria-label", demo.alt);
  videoElement.addEventListener(
    "loadeddata",
    () => {
      if (!showLoader) {
        videoElement.classList.remove("is-loading");
        return;
      }

      const elapsed = performance.now() - videoLoadingStartedAt;
      const remainingVisibleTime = Math.max(
        0,
        MIN_VIDEO_LOADING_VISIBLE_MS - elapsed,
      );

      window.setTimeout(() => {
        hideVideoLoading();
        window.setTimeout(() => {
          videoElement.classList.remove("is-loading");
          copyElement?.classList.remove("is-loading");
          setCopyText(demo.title, demo.description);
        }, VIDEO_LOADING_FADE_MS);
      }, remainingVisibleTime);
    },
    { once: true },
  );
  videoElement.load();
  void videoElement.play().catch(() => {});
}

function setVideoVisibility(videoElement, visible) {
  if (!(videoElement instanceof HTMLVideoElement)) {
    return;
  }

  videoElement.classList.toggle("is-visible", visible);
  videoElement.classList.toggle("is-hidden", !visible);
}

function renderCopy(demo) {
  if (!demo) {
    return;
  }

  setCopyText(demo.title, demo.description);
}

function renderDemo(index) {
  const demo = demos[index];
  if (!demo) {
    return;
  }

  applyDemo(primaryVideo, demo, hasInitializedDemo);
  setVideoVisibility(primaryVideo, true);
  setVideoVisibility(secondaryVideo, false);
  renderCopy(demo);
  hasInitializedDemo = true;
}

function switchDemo(direction) {
  if (isAnimating || demos.length === 0) {
    return;
  }

  const currentVideo = showingPrimary ? primaryVideo : secondaryVideo;
  const nextVideo = showingPrimary ? secondaryVideo : primaryVideo;

  isAnimating = true;
  currentIndex = (currentIndex + direction + demos.length) % demos.length;
  const demo = demos[currentIndex];

  if (
    !(currentVideo instanceof HTMLVideoElement) ||
    !(nextVideo instanceof HTMLVideoElement) ||
    !demo
  ) {
    renderDemo(currentIndex);
    isAnimating = false;
    return;
  }

  copyElement?.classList.add("is-switching");
  renderCopy(demo);
  applyDemo(nextVideo, demo);
  setVideoVisibility(nextVideo, true);
  setVideoVisibility(currentVideo, false);

  setTimeout(() => {
    copyElement?.classList.remove("is-switching");
    showingPrimary = !showingPrimary;
    isAnimating = false;
  }, SLIDE_TRANSITION_MS);
}

renderDemo(currentIndex);

if (primaryVideo instanceof HTMLVideoElement) {
  primaryVideo.addEventListener("loadeddata", lockFrameSize, { once: true });
}

window.addEventListener("load", () => {
  loadingScreen?.classList.add("is-hidden");
});

leftButton?.addEventListener("click", () => switchDemo(-1));
rightButton?.addEventListener("click", () => switchDemo(1));
primaryVideo?.addEventListener("click", () => switchDemo(1));
secondaryVideo?.addEventListener("click", () => switchDemo(1));

document.addEventListener("keydown", (event) => {
  if (event.key === "ArrowLeft") {
    switchDemo(-1);
  } else if (event.key === "ArrowRight") {
    switchDemo(1);
  }
});
