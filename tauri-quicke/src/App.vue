<template>
  <div class="annular">
      <div class="annular_container">
          <div
              v-for="(item, index) in menuList"
              :key="index"
              class="annular_container_item"
              :style="{transform: `rotate(${360 / menuList.length * index}deg)`}"
              @click="itemClick(item, index)"
          >
              <span :style="{transform: `rotate(${-360 / menuList.length * index}deg)`}">{{ item.name }}</span>
          </div>
      </div>
      <div class="annular_center">关闭</div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/shell';

const menuList = [
    { name: '百度', url: 'https://www.baidu.com' },
    { name: '百度', url: 'https://www.baidu.com' },
    { name: '百度', url: 'https://www.baidu.com' },
    { name: '百度', url: 'https://www.baidu.com' },
    { name: '百度', url: 'https://www.baidu.com' },
    { name: '百度', url: 'https://www.baidu.com' },
    { name: '百度', url: 'https://www.baidu.com' },
    { name: '百度', url: 'https://www.baidu.com' },
];

const rightHandler = async (payload) => {
  await invoke('window_show', { show: true, x: payload.x, y: payload.y});
}

// 监听h
const eventListener = async () => {
  await listen('buttonDown', (event) => {
    const { payload } = event;
    if (payload.button === 'Right') {
      rightHandler(payload)
    }
    console.log(`Got error in window ${event.windowLabel}, payload: ${JSON.stringify(event.payload)}`);
  });
}


const close = async () => {
  await invoke('window_show', { show: false });
}

const itemClick = async (item) => {
  await close();
  if (item.url) {
    await open(item.url);
  }
}

onMounted(() => {
  eventListener()
})

</script>

<style lang="less" scoped>
@blur-width: 5px;
@app-padding: 4 * @blur-width;
@app-width: calc(min(100vh, 100vw) - 2 * @app-padding);
@app-width-half: calc(min(100vh, 100vw)/2 - @app-padding);
@center-width: 100px;
@bg-color: blanchedalmond;
.annular {
  height: 100%;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  font-size: 18px;
  color: #fff;
  border-radius: 50%;
  user-select: none;
  -webkit-user-select: none;
  &_container {
      width: 100%;
      height: 100%;
      border-radius: 50%;
      box-shadow: 0 0 @blur-width;
      width: @app-width;
      height: @app-width;
      position: absolute;
      left: @app-padding;
      top: @app-padding;
      overflow: hidden;
      background-color: rgba(24, 24, 24, .35);
      &_item {
          position: absolute;
          width: 100%;
          height: 100%;
          border-radius: 50%;
          background-color: rgba(24, 24, 24, .75);
          cursor: pointer;
          z-index: 1;
          transition: all .5s;
          // 8个
          clip-path: polygon(50% 50%, 100% 13px, 100% 0, 53% 0);
          // 4个
          // clip-path: polygon(50% 50%, 100% 191px, 100% 0, 50% 0);
          &:hover {
            background-color: green;
          }
          span {
              position: absolute;
              left: 60%;
              top: 10%;
          }
      }
  }

  &_center {
      position: absolute;
      top: calc(@app-width-half - @center-width/2 + 10px);
      width: @center-width;
      height: @center-width;
      border-radius: 50%;
      border: 6px solid #FFD824;
      background-color: black;
      display: flex;
      align-items: center;
      justify-content: center;
      box-shadow: -15px -15px 65px #010711;
      transition: all .5s;
      z-index: 1;
      &:hover {
        background-color: red;
      }
  }
}
</style>
