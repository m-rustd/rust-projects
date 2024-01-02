<template>
  <div class="container">
    <div class="app circle">
      <video
        ref="videoInputRef"
        id="cam-video"
        class="cam-video cam-play"
        controlslist="nodownload nofullscreen noremoteplayback"
      />
      <canvas class="cam-video cam-video-revert" id="canvasOutput"></canvas>
    </div>
    <div class="app app-circle-no-hidden"
      id="detect-show-area"
      data-tauri-drag-region
      @mouseenter="handleMouseEnter"
      @mouseleave="handleMouseLeave"
    >
    {{showAllButton}}
      <div data-btn class="btn btn-close" :class="{ 'hide-circle-btn': !showAllButton }" @click="closeHandler">
        <img src="./assets/close.svg" />
      </div>
      <div data-btn class="btn btn-beauty" :class="[{ 'hide-circle-btn': !showAllButton }, {'btn-selected': beauty}]" @click="beautyHandler">
        <img src="./assets/beauty.svg" />
      </div>
      <div data-btn class="btn btn-mirror" :class="[{ 'hide-circle-btn': !showAllButton }, {'btn-selected': mirror}]" @click="mirrorHandler">
        <img src="./assets/mirror.svg" />
      </div>
      <div data-btn class="btn btn-face" :class="[{ 'hide-circle-btn': !showAllButton }, {'btn-selected': face}]" @click="faceHandler">
        <img src="./assets/face.svg" />
      </div>

      <div data-btn class="btn-change-size" :class="{ 'hide-circle-btn': !showAllButton }">
        <div class="btn-change-size-btn btn-change-size-small" :class="{'btn-change-size-select': btnSize === 0}" @click="btnSizeHandler('small')"></div>
        <div class="btn-change-size-btn btn-change-size-middle" :class="{'btn-change-size-select': btnSize === 1}"  @click="btnSizeHandler('middle')"></div>
        <div class="btn-change-size-btn btn-change-size-large" :class="{'btn-change-size-select': btnSize === 2}" @click="btnSizeHandler('large')"></div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue';
import { Utils } from './common/cv-util';

const utils = new Utils();
const videoInputRef = ref(null);
const beauty = ref(false);
const face = ref(false);
const mirror = ref(false);
const showAllButton = ref(false);
const btnSize = ref(0);

// const { dialog } = window.__TAURI__;
// const { LogicalPosition, LogicalSize, getCurrent, appWindow } = window.__TAURI__.window;

const RUNTIME_CONF = {
  fps: true,
  fps_dom: null,
  face: false,
  beauty: false,
};

const isTauri = window.__TAURI__;

const closeHandler = () => {
  if (isTauri) {
    window.__TAURI__.window.appWindow.close();
  }
}

const beautyHandler = () => {
  beauty.value = !beauty.value;
  console.log('beauty---------');
}

const faceHandler = () => {
  face.value = !face.value;
  console.log('face---------');
}

const mirrorHandler = () => {
  mirror.value = !mirror.value;
  console.log('mirror---------');
}

// 当鼠标进入时触发的函数
const handleMouseEnter = () => {
  showAllButton.value = true;
}

// 当鼠标离开时触发的函数
const handleMouseLeave = () => {
  showAllButton.value = false;
}

const btnSizeHandler = (size) => {
  switch(size) {
    case 'small':
      btnSize.value = 0;
      break;
    case 'middle':
      btnSize.value = 1;
      break;
    case 'large':
      btnSize.value = 2;
      break;
    default: 
      break;
  }
}

function onVideoStarted() {
    const videoInput = videoInputRef.value;
    videoInput.width = videoInput.videoWidth;
    videoInput.height = videoInput.videoHeight;

    let src = new cv.Mat(videoInput.height, videoInput.width, cv.CV_8UC4);
    let dst = new cv.Mat(videoInput.height, videoInput.width, cv.CV_8UC4);
    let gray = new cv.Mat();
    let cap = new cv.VideoCapture(videoInput);
    let faces = new cv.RectVector();
    let classifier = new cv.CascadeClassifier();
    let [roiPoint1, roiPoint2] = [
      { x: 0, y: 0 },
      {
        x: videoInput.width,
        y: videoInput.height,
      },
    ];

    // classifier.load("haarcascade_frontalface_default.xml");

    const FPS = 30;
    function processVideo() {
      try {
        let begin = Date.now();

        cap.read(src);
        src.copyTo(dst);

        let renderMat = dst;

        if (RUNTIME_CONF.face) {
          cv.cvtColor(dst, gray, cv.COLOR_RGBA2GRAY, 0);
          cv.pyrDown(gray, gray);
          cv.pyrDown(gray, gray);
          classifier.detectMultiScale(gray, faces, 1.1, 3, 0);

          let face = faces.get(0);

          if (face) {
            let xRatio = videoInput.width / gray.size().width;
            let yRatio = videoInput.height / gray.size().height;

            // 扩大人脸范围
            [roiPoint1, roiPoint2] = utils.face_padding(
              {
                x: face.x * xRatio,
                y: face.y * yRatio,
              },
              {
                x: face.width * xRatio,
                y: face.height * yRatio,
              },
              videoInput.width,
              videoInput.height
            );
          }

          renderMat = dst.roi(
            new cv.Rect(
              ...utils.shakeFilter(
                { x: roiPoint1.x, y: roiPoint1.y },
                { x: roiPoint2.x, y: roiPoint2.y }
              )
            )
          );
        }

        if (RUNTIME_CONF.beauty) {
          renderMat = utils.beauty(renderMat);
        }

        cv.imshow("canvasOutput", renderMat);

        let delay = 1000 / FPS - (Date.now() - begin);
        setTimeout(processVideo, delay);
      } catch (err) {
        utils.printError(err);
      }
    }

    setTimeout(processVideo, 0);
  }

const loadOpenCV = () => {
  utils.loadOpenCv(() => {
    utils.startCamera("", onVideoStarted, "cam-video", async () => {
      if (isTauri) {
        const { dialog } = window.__TAURI__;
        const { appWindow } = window.__TAURI__.window;
        await dialog.message("本应用需要摄像头授权，否则无法使用", {
          title: "授权失败",
          type: "error",
        });
        appWindow.close();
      }
    });
    // utils.createFileFromUrl(
    //   "haarcascade_frontalface_default.xml",
    //   "./assets/haarcascade_frontalface_default.xml",
    //   () => {
    //     utils.startCamera("", onVideoStarted, "cam-video", async () => {
    //       await dialog.message("本应用需要摄像头授权，否则无法使用", {
    //         title: "授权失败",
    //         type: "error",
    //       });
    //       appWindow.close();
    //     });
    //   }
    // );
  });
}

const keyDownHandler = (e) => {
  if ((e.ctrlKey || e.metaKey) && e.keyCode === 70) {
    e.preventDefault();
    RUNTIME_CONF.fps = !RUNTIME_CONF.fps;
  }
}

onMounted(() => {
  console.log('=====', videoInputRef.value)
  loadOpenCV();
  const stats = new Stats();
  const loop = function loop() {
    if (RUNTIME_CONF.fps) {
      if (!RUNTIME_CONF.fps_dom) {
        console.log('------')
        document.body.appendChild(stats.dom);
        RUNTIME_CONF.fps_dom = stats.dom;
      }
      RUNTIME_CONF.fps_dom.style.display = "block";
    } else if (!RUNTIME_CONF.fps) {
      if (RUNTIME_CONF.fps_dom) {
        RUNTIME_CONF.fps_dom.style.display = "none";
      }
    }

    RUNTIME_CONF.fps && stats.update();
    requestAnimationFrame(loop);
  };

  loop();
  window.addEventListener("keydown", keyDownHandler, true);
})
</script>

<style scoped lang="less">
@blur-width: 5px;
@app-padding: 4 * @blur-width;
@app-width: calc(min(100vh, 100vw) - 2 * @app-padding);
.container {
  position: relative;
  height: 100%;
  width: 100%;
  border-radius: 50%;
  .app {
    position: absolute;
    width: @app-width;
    height: @app-width;
    z-index: 1;
    left: @app-padding;
    top: @app-padding;
  }
  .circle {
    border-radius: 50%;
    overflow: hidden;
    box-shadow: 0 0 @blur-width;

    .cam-play {
      visibility: hidden;
      position: absolute;
    }
    .cam-video {
      transform: translateX(-50%);
      height: 100%;
      margin-left: 50%;
    }

    .cam-video-revert {
      transform: translateX(-50%) scaleX(-1);
    }
  }
  [data-tauri-drag-region]:hover {
    cursor: move;
  }
  .app-circle-no-hidden {
      border-radius: 50%;
      box-shadow: 0 0 @blur-width;
      .btn {
        position: absolute;
        cursor: pointer;
        user-select: none;
        -webkit-user-select: none;
        width: 40px;
        height: 40px;
        border-radius: 50%;
        background-color: #fff;
        box-shadow: 0 0 5px;
        display: flex;
        justify-content: center;
        align-items: center;
        box-sizing: border-box;
        padding: 5px;
      }

      .btn-selected {
        background-color: #333;
      }

      .btn-selected img {
        filter: invert(1) !important;
      }

      .btn img {
        width: 100%;
        height: 100%;
        pointer-events: none;
      }

      .btn-close {
        right: calc(@app-width / 8);
        top: calc(@app-width / 8);
        transform: translate(50%, -50%);
      }

      .btn-beauty {
        right: calc(@app-width / 8);
        bottom: calc(@app-width / 8);
        transform: translate(50%, 50%);
      }

      .btn-face {
        left: calc(@app-width / 8);
        bottom: calc(@app-width / 8);
        transform: translate(-50%, 50%);
      }

      .btn-mirror {
        left: calc(@app-width / 8);
        top: calc(@app-width / 8);
        transform: translate(-50%, -50%);
      }

      .hide-circle-btn {
        display: none !important;
      }

      .btn-change-size {
        width: 80px;
        height: 38px;
        backdrop-filter: blur(20px);
        left: 0px;
        right: 0px;
        position: absolute;
        bottom: 20px;
        background: rgba(0, 0, 0, 0.5);
        border-radius: 30px;
        margin: auto;
        display: flex;
        justify-content: center;
        align-items: center;
        cursor: default;
        user-select: none;
        -webkit-user-select: none;
      }
      .btn-change-size-btn {
        border: 1px solid #fff;
        border-radius: 50%;
        user-select: none;
        -webkit-user-select: none;
        margin-left: 5px;
        cursor: pointer;
      }
      .btn-change-size-small {
        width: 9px;
        height: 9px;
      }
      .btn-change-size-middle {
        width: 14px;
        height: 14px;
      }
      .btn-change-size-large {
        width: 20px;
        height: 20px;
      }
      .btn-change-size-select {
        background-color: #fff;
      }
    }
}
</style>
