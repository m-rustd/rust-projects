<!--
 * @Author: misterzhou
 * @Date: 2024-01-12 13:35:50
 * @LastEditTime: 2024-01-12 13:35:56
 * @LastEditors: misterzhou
 * @FilePath: /tauri-quicke/src/App copy.vue
 * @Description: 
-->
<script setup>
import { computed, onMounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/shell';

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

const panel = ref([
  {img:'https://tauri.app/img/index/header_light.svg',title:'百度', url: 'https://www.baidu.com'},
  {img:'https://tauri.app/img/index/header_light.svg',title:'一月石'},
  {img:'https://tauri.app/img/index/header_light.svg',title:'一月石'},
  {img:'https://tauri.app/img/index/header_light.svg',title:'一月石'},
  {img:'https://tauri.app/img/index/header_light.svg',title:'一月石'},
  // {img:'https://tauri.app/img/index/header_light.svg',title:'一月石榴石'},
  // {img:'https://tauri.app/img/index/header_light.svg',title:'一月石榴石'},
  // {img:'pics-gem/4.png',title:'一月石榴石'},
  // {img:'pics-gem/5.png',title:'一月石榴石'},
  // {img:'pics-gem/6.png',title:'一月石榴石'},
  // {img:'pics-gem/7.png',title:'一月石榴石'},
  // {img:'pics-gem/8.png',title:'一月石榴石'},
  // {img:'pics-gem/9.png',title:'一月石榴石'},
  // {img:'pics-gem/10.png',title:'一月石榴石'},
  // {img:'pics-gem/11.png',title:'一月石榴石'},
  // {img:'pics-gem/12.png',title:'一月石榴石'}
]);

const close = async () => {
  await invoke('window_show', { show: false });
}

const itemClick = async (item) => {
  await close();
  if (item.url) {
    await open(item.url);
  }
}

const eveDeg = computed(() => {
  if (panel.value.length > 0) {
    return 360/panel.value.length;
  }
  return 30;
})

onMounted(() => {
  eventListener()
})

</script>

<template>
  <div class="container">
    <div class="menu-wrap">
      <ul>
        <li v-for="(item,index) in panel" :key="index" :style="{transform: `rotate(${-10 + eveDeg*index}deg) skew(60deg)`}" @click="itemClick(item)">
          <a href="#">
            <img class="li-img" :src="item.img" alt />
            <div class="li-text">{{item.title}}</div>
          </a>
        </li>
      </ul>
      <div class="center-wrap" @click="close">关闭</div>
    </div>
  </div>
</template>

<style scoped lang="less">
@blur-width: 5px;
@app-padding: 4 * @blur-width;
@app-width: calc(min(100vh, 100vw) - 2 * @app-padding);
@app-width-half: calc(min(100vh, 100vw)/2 - @app-padding);
@center-width: 130px;
@bg-color: blanchedalmond;
.container {
  position: relative;
  height: 100%;
  width: 100%;
  border-radius: 50%;
  user-select: none;
  -webkit-user-select: none;
  .menu-wrap {
    background-color: @bg-color;
    position: absolute;
    width: @app-width;
    height: @app-width;
    z-index: 1;
    left: @app-padding;
    top: @app-padding;
    overflow: hidden;
    border-radius: 50%;
    box-shadow: 0 0 @blur-width;
    transform: scale(1);
    width: @app-width;
    height: @app-width;
    font-size: 16px;
    li {
      position: absolute;
      font-size: 1.2em;
      width: 10em;
      height: 10em;
      -webkit-transform-origin: 100% 100%;
      -moz-transform-origin: 100% 100%;
      -ms-transform-origin: 100% 100%;
      transform-origin: 100% 100%;
      overflow: hidden;
      left: 50%;
      margin-top: -2em;
      margin-left: -10em;
      -webkit-transition: border 0.3s ease;
      -moz-transition: border 0.3s ease;
      transition: border 0.3s ease;
    }

    li a {
      display: flex;
      flex-direction: column;
      // justify-content: center;
      align-items: center;
      font-size: 1.18em;
      height: 14.5em;
      width: 14.5em;
      border-radius: 50%;
      text-decoration: none;
      color: #fff;
      padding-top: 60px;
      text-align: center;
      -webkit-transform: skew(-60deg) rotate(-70deg) scale(1);
      -ms-transform: skew(-60deg) rotate(-70deg) scale(1);
      -moz-transform: skew(-60deg) rotate(-70deg) scale(1);
      transform: skew(-60deg) rotate(-70deg) scale(1);
      -webkit-transition: opacity 0.3s, color 0.3s;
      -moz-transition: opacity 0.3s, color 0.3s;
      transition: opacity 0.3s, color 0.3s;
    }
    li:nth-child(odd) a {
      background-color: #a11313;
      background-color: hsla(0, 88%, 63%, 1);
    }

    li:nth-child(even) a {
      background-color: #a61414;
      background-color: hsla(0, 88%, 65%, 1);
    }

    li a {
      background-color: rgba(135, 137, 155, 0.52);
      border: solid 0px #f2cc81;
      &:hover {
        background-color: green;
      }
    }

    /* hover style */
    // li:not(.active) a:hover,
    // li:not(.active) a:active,
    // li:not(.active) a:focus {
    //   background-color: rgba(135, 137, 155, 0.52);
    //   border: solid 0px #f2cc81;
    // }

    .li-img {
      width: 50px;
      margin-bottom: 10px;
      margin-left: -10px;
    }
    .li-text {
      color: #f2cc81;
      font-size: 16px;
      width: 76px;
      margin: 0 calc(50% - 50px);
    }
  }
  .center-wrap {
    position: absolute;
    left: calc(@app-width-half - @center-width/2);
    top: calc(@app-width-half - @center-width/2);
    z-index: 2;
    width: @center-width;
    height: @center-width;
    border-radius: 50%;
    box-shadow: 0 0 @blur-width;
    background-color: @bg-color;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
    user-select: none;
    
    &:hover {
      background-color: red;
    }
  }
}
</style>
