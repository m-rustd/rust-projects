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
  {img:'https://tauri.app/img/index/header_light.svg',title:'百度', url: 'https://www.baidu.com', color: 'red'},
  // {img:'https://tauri.app/img/index/header_light.svg',title:'一月石', color: 'green'},
  // {img:'https://tauri.app/img/index/header_light.svg',title:'一月石', color: 'blue'},
  // {img:'https://tauri.app/img/index/header_light.svg',title:'一月石', color: 'yellow'},
  // {img:'https://tauri.app/img/index/header_light.svg',title:'一月石' , color: 'pink'},
  // {img:'https://tauri.app/img/index/header_light.svg',title:'一月石', color: 'black'},
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

const getRandomColor = function() {
    return "rgb(" + Math.round(Math.random() * 255) + "," + Math.round(Math.random() * 255) + ',' + Math.round(Math.random() * 10) + ')';
}

const size = 140;
const angle = 60;

// 绘制N个扇形的起始位置
const pointListWithCount = (count) => {
  const pointArr = [];
  const currentPoint = { startX: 0, startY: -size, endX: 0, endY: -size };
  let currentAngle = angle;
  Array.from({length: count}).forEach(_=> {
    currentPoint.startX = currentPoint.endX;
    currentPoint.startY = currentPoint.endY;
    // 判断象限
    const quadrant = currentAngle / 90;
    // 是否为边界
    const isLineInteger = (quadrant === 1 || quadrant === 2) ? 1 : -1;
    switch(quadrant) {
      case 1:
        currentPoint.endX = isLineInteger*size;
        currentPoint.endY = 0;
        break;
      case 2:
        currentPoint.endX = 0;
        currentPoint.endY = isLineInteger*size;
        break;
      case 3:
        currentPoint.endX = isLineInteger*size;
        currentPoint.endY = 0;
        break;
      case 4:
        currentPoint.endX = 0;
        currentPoint.endY = isLineInteger*size;
        break;
      default:
        break;
    }
    const deg = currentAngle/180*Math.PI;
    const imgDeg = currentAngle/180*Math.PI;
    // 第一象限
    if (quadrant < 1) {
      currentPoint.endX = Math.floor(Math.abs(Math.sin(deg)*size));
      currentPoint.endY = -Math.floor(Math.abs(Math.cos(deg))*size);
    }
    // 第二象限
    else if (quadrant <= 2) {
      currentPoint.endX = Math.floor(Math.abs(Math.sin(deg)*size));
      currentPoint.endY = Math.floor(Math.abs(Math.cos(deg))*size);
    }
    // 第三象限
    else if (quadrant <= 3) {
      currentPoint.endX = -Math.floor(Math.abs(Math.sin(deg)*size));
      currentPoint.endY = Math.floor(Math.abs(Math.cos(deg))*size);
    }
    // 第四象限
    else if (quadrant <= 4) {
      currentPoint.endX = -Math.floor(Math.abs(Math.sin(deg)*size));
      currentPoint.endY = -Math.floor(Math.abs(Math.cos(deg))*size);
    }

    // // 第一象限
    // if (currentAngle <= 90) {
    //   if (currentAngle === 90) {
    //     currentPoint.endX = size;
    //     currentPoint.endY = 0;
    //   } else {
    //     currentPoint.endX = Math.floor(Math.abs(Math.sin(deg)*size));
    //     currentPoint.endY = -Math.floor(Math.abs(Math.cos(deg))*size);
    //   }
    // }
    // // 第二象限
    // else if (currentAngle <= 180) {
    //   if (currentAngle === 180) {
    //     currentPoint.endX = 0;
    //     currentPoint.endY = size;
    //   } else {
    //     currentPoint.endX = Math.floor(Math.abs(Math.sin(deg)*size));
    //     currentPoint.endY = Math.floor(Math.abs(Math.cos(deg))*size);
    //   }
    // }
    // // 第三象限
    // else if (currentAngle <= 270) {
    //   if (currentAngle === 270) {
    //     currentPoint.endX = -size;
    //     currentPoint.endY = 0;
    //   } else {
    //     currentPoint.endX = -Math.floor(Math.abs(Math.sin(deg)*size));
    //     currentPoint.endY = Math.floor(Math.abs(Math.cos(deg))*size);
    //   }
    // }
    // // 第四象限
    // else if (currentAngle <= 360) {
    //   if (currentAngle === 360) {
    //     currentPoint.endX = 0;
    //     currentPoint.endY = -size;
    //   } else {
    //     currentPoint.endX = -Math.floor(Math.abs(Math.sin(deg)*size));
    //     currentPoint.endY = -Math.floor(Math.abs(Math.cos(deg))*size);
    //   }
    // }
    currentAngle += angle;
    pointArr.push({...currentPoint, bgColor: getRandomColor(), img: 'https://tauri.app/img/index/header_light.svg', title: 'baidu'});
  });
  return pointArr;
}

const points = computed(() => {
  return pointListWithCount(360/angle);
})

onMounted(() => {
  eventListener()
  // document.addEventListener('mousemove', (e) => {
  //   console.log('$---', e);
  // })
})

</script>

<template>
  <div class="container">
    <div class="menu-wrap">
      <!-- <svg :width="size*2" :height="size*2" xmlns="http://www.w3.org/2000/svg"> 
        <g :transform="`translate(${size},${size})`" v-for="(item, index) in points" :key="index" @mouseup="itemClick">
          <path :d="`M0,0 L${item.startX},${item.startY} A${size},${size},0,0,1,${item.endX},${item.endY} Z`" :fill="item.bgColor"/>
          <image :x="item.startX/2-30" :y="(item.startY+item.endY)/2-20" width="25" xlink:href="./assets/vue.svg"></image>
          <text class='item-text' :x="Math.floor(Math.abs(Math.sin(30/180*Math.PI)*size/2))" :y="Math.floor(Math.abs(Math.cos(30/180*Math.PI)*size/2))">
            {{item.title}}
          </text>
        </g> 
      </svg> -->
    </div>
    <div class="center-wrap" @click="close">关闭</div>
  </div>
</template>

<style scoped lang="less">
@blur-width: 5px;
@app-padding: 4 * @blur-width;
@app-width: calc(min(100vh, 100vw) - 2 * @app-padding);
@app-width-half: calc(min(100vh, 100vw)/2 - @app-padding);
@center-width: 100px;
@bg-color: blanchedalmond;
.container {
  user-select: none;
  -webkit-user-select: none;
  display: flex;
  justify-content: center;
  align-items: center;
  .menu-wrap {
    display: flex;
    justify-content: center;
    align-items: center;
    position: relative;
    width: @app-width;
    height: @app-width;
    overflow: hidden;
    border-radius: 50%;
    box-shadow: 0 0 @blur-width;
    .center-wrap {
      display: flex;
      justify-content: center;
      align-items: center;
    }
    svg {
      g {
        p {
          &:hover {
            background-color: red;
          }
        }
      }
    }
  }
  .center-wrap {
    position: absolute;
    width: @center-width;
    height: @center-width;
    border-radius: 50%;
    box-shadow: 0 0 @blur-width;
    background-color: @bg-color;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
    &:hover {
      background-color: red;
    }
  }
}
</style>
