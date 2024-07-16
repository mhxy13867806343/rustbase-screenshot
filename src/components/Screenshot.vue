<script setup>






const props=defineProps({
	data:{
		type:Object,
		default:()=>{}
	}
})
const statusComputed = computed(() => {
    let transform = {
        transition: "transform 0.5s ease-in-out",  // 添加 easing function
        transform: "rotate(0deg)"    // 初始化默认值
    };

    if (props.data && props.data.angleDeg) {
        transform.transform = `rotate(${props.data.angleDeg}deg)`;  // 使用 transform 属性
    }

    return transform;
});

</script>

<template>
	<div class="screenshot" :style="{...statusComputed}">
		screenshot
	</div>
</template>

<style scoped>
.screenshot{
	width: 100%;
	max-height: 300px;
	height: 300px;
	background: red;
}
</style>
