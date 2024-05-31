<template>
  <div style="height: 100vh; position: relative;overflow: hidden;">
    <n-layout position="absolute">
      <n-layout-header style="height: 64px; padding: 24px" bordered>
        颐和园路
      </n-layout-header>
      <n-layout has-sider position="absolute" style="top: 64px; bottom: 0">
        <n-layout-sider bordered content-style="padding: 24px;">
          <n-button @click="toGrey" style="margin-bottom: 10px;">去色</n-button>
          <n-form ref="formRef" :label-width="80">
            <n-form-item label="亮度" path="user.name">
              <n-slider v-model:value="num" :step="1" :max="100" :min="-100" :on-dragend="bright" />
              <span style="margin-left: 10px;">{{ num }}</span>
            </n-form-item>
            <n-form-item label="对比度" path="user.name">
              <n-slider v-model:value="num2" :step="1" :max="100" :min="-100" :on-dragend="bright" />
              <span style="margin-left: 10px;">{{ num2 }}</span>
            </n-form-item>
            <n-form-item label="模糊" path="user.name">
              <n-slider v-model:value="num3" :step="1" :max="100" :min="-100" :on-dragend="bright" />
              <span style="margin-left: 10px;">{{ num3 }}</span>
            </n-form-item>
          </n-form>
        </n-layout-sider>
        <n-layout content-style="padding: 24px; display:flex; justify-content:center;align-items:center">
          <canvas ref="canvasRef" style="width: 600px; height: 100px"></canvas>
        </n-layout>
      </n-layout>
    </n-layout>
  </div>
  <!-- <div>
    <canvas ref="canvasRef" style="width: 600px; height: 100px"></canvas>
    <button @click="toGrey">灰色</button>
    <input type="text" v-model="num">
    <n-button @click="bright">测试</n-button>
  </div> -->
</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import carImage from "./assets/car2.jpg";
import init, { canvas_to_gray, brighten2, contrast, adjust, AdjustParams } from 'wasm-image-filter';

const canvasRef = ref<HTMLCanvasElement>();
const carImageRef = ref<HTMLImageElement>();

const toGrey = async () => {
  const ctx = canvasRef.value?.getContext("2d");
  const imageData = ctx?.getImageData(0, 0, ctx.canvas.width, ctx.canvas.height)
  console.log(imageData);

  try {
    const newImageData = canvas_to_gray(imageData!);
    ctx?.putImageData(newImageData, 0, 0)
  } catch (error) {
    console.log(error);

  }
}

const num = ref(0)
const bright = () => {
  const ctx = canvasRef.value?.getContext("2d");
  const imageData = ctx?.getImageData(0, 0, ctx.canvas.width, ctx.canvas.height)
  try {
    const adjustPrams = new AdjustParams()
    adjustPrams.image_data = orginCanvas
    adjustPrams.brightness = Number(num.value)
    adjustPrams.constrast_radio = Number(num2.value)
    adjustPrams.blur = Number(num3.value)
    const newImageData = adjust(adjustPrams);
    console.log(newImageData);
    // const data = brighten2(orginCanvas, 2)
    // console.log(orginCanvas);
    // console.log(newImageData);

    // var newImage = new ImageData(ctx!.canvas.width, ctx!.canvas.height);
    // newImage.data.set(data)

    ctx?.putImageData(newImageData, 0, 0)
  } catch (error) {
    console.log(error);

  }
}

const num2 = ref(0)
const num3 = ref(0)
const adjustContrast = () => {
  const ctx = canvasRef.value?.getContext("2d");
  const imageData = ctx?.getImageData(0, 0, ctx.canvas.width, ctx.canvas.height)
  try {
    const newImageData = contrast(orginCanvas, Number(num2.value));
    ctx?.putImageData(newImageData, 0, 0)
  } catch (error) {
    console.log(error);

  }
}



let orginCanvas: any = null;
onMounted(async () => {
  await init()
  const img = new Image();
  img.src = carImage;

  img.onload = () => {
    console.log("成功");

    setTimeout(() => {
      const width = 600;
      const height = (600 * img.height) / img.width
      const ctx = canvasRef.value?.getContext("2d");
      // console.log("ctx:", ctx);
      canvasRef.value!.style.height = (600 * img.height) / img.width + "px";
      ctx!.canvas.width = width;
      ctx!.canvas.height = height;
      ctx?.drawImage(img, 0, 0, width, height);
      orginCanvas = ctx?.getImageData(0, 0, ctx.canvas.width, ctx.canvas.height)
    }, 1000);
  };
});
</script>

<style>
html,
body {
  width: 100%;
  height: 100%;
  overflow: hidden;
}
</style>
