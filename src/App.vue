<script setup>
import { ElMessage } from 'element-plus'
import { LogicalSize, appWindow } from '@tauri-apps/api/window';
import{open,save} from "@tauri-apps/api/dialog"
import {  writeFile } from '@tauri-apps/api/fs'; // 引入 writeFile
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted } from 'vue';
const angleDeg=ref(0)
const _angleDeg=90
const scale=ref(1)
const _scale=1
const timestamp=ref("")
const sizeValue=ref({})
onMounted(async ()=>{
  getWindowSize()
  await savePathUrl()
})
const savePathUrl=async ()=>{
  const result=await invoke("savePathUrl",{
        fileName:new Date(),
      })
      timestamp.value=result.timestamp||new Date()
}
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
const onClickSave = async () => {
  await savePathUrl()
  try {
    const path = await save({
      title: "保存文件",
      filters: [
        { name: "请选择文件后缀", extensions: ["txt", "jpg","gif","web"] },
        { name: "请选择", extensions: ["*"] }
      ],
	  defaultPath:`文件名${timestamp.value}`
    });
	console.log(path,222)
    if (path) {
    
      // 使用 writeFile 写入内容到文件
      await writeFile({
        path,
        contents: result.timestamp
      });
      ElMessage.info(`文件已保存到：${path}`);
    } else {
      ElMessage.error(`保存操作被取消`);
    }
  } catch (error) {
    ElMessage.error(`保存文件出错: ${error}`);
  }
}
const onClickLeft=async ()=>{
  console.log("left")
  const direction=await invoke("setAngleDeg",{
    direction:"Left"
  })
  angleDeg.value-=_angleDeg
}
const onClickRight=async ()=>{
  console.log("right")
  const direction=await invoke("setAngleDeg",{
    direction:"Right"
  })
  angleDeg.value+=_angleDeg
  
 
}
const onClickScaleAdd=()=>{
  console.log('add')
  const {width,height}=sizeValue.value
  if(width>=700 && height>=700){
    ElMessage.error("不能再放大了")
    return 
  }
  enlargeWindow()
}
const onClickScaleSub=()=>{
  console.log('sub')
  const {width,height}=sizeValue.value
  if(width<=320 && height<=500){
    ElMessage.error("不能再缩小了")
    return 
  }
  if(scale.value<=1){
    ElMessage.error("不能再缩小了")
    return 
  }
  reduceWindow(scale.value)
}
async function getWindowSize() {
  try {
    const size = await appWindow.innerSize();
    console.log(`窗口大小：宽度=${size.width}, 高度=${size.height}`);
    sizeValue.value=size
  } catch (error) {
    ElMessage.error('获取窗口大小时发生错误:', error);
  }
}
async function enlargeWindow() {
  // 设定想要放大到的尺寸
  const newSize = new LogicalSize(600, 600);
  await appWindow.setSize(newSize);
}

async function reduceWindow() {
  // 设定想要缩小到的尺寸
  const newSize = new LogicalSize(400, 400);
  await appWindow.setSize(newSize);
}
</script>

<template>
	
	<el-button type="success">Success</el-button>
	<div class="more-tools">
		<svg t="1720512277464" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="13132" width="200" height="200"><path d="M224 512m-80 0a80 80 0 1 0 160 0 80 80 0 1 0-160 0Z" fill="#bfbfbf" p-id="13133"></path><path d="M512 512m-80 0a80 80 0 1 0 160 0 80 80 0 1 0-160 0Z" fill="#bfbfbf" p-id="13134"></path><path d="M800 512m-80 0a80 80 0 1 0 160 0 80 80 0 1 0-160 0Z" fill="#bfbfbf" p-id="13135"></path></svg>
	</div>
	<Screenshot :data="{
    angleDeg
  }"/>
	<Operate  @save="onClickSave"
  @left="onClickLeft"
  @right="onClickRight"
  @scale-sub="onClickScaleSub"
  @scale-add="onClickScaleAdd"
  />
</template>

<style>
:root{
	--flex--:flex;
	--flex-row--:row;
	--justify-content-space-around--:space-around;
	--justify-content-flex-end--:flex-end;
	--align-items-center--:center;
}
.more-tools{
	width: 100%;
	display: var(--flex--);
	justify-content: var(--justify-content-flex-end--);
}

.icon{
	width: 30px;
	height: 30px;
}
</style>
