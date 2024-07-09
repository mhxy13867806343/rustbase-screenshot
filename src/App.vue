<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
</script>

<template>
	
	<el-button type="success">Success</el-button>
	<div class="more-tools">
		<svg t="1720512277464" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="13132" width="200" height="200"><path d="M224 512m-80 0a80 80 0 1 0 160 0 80 80 0 1 0-160 0Z" fill="#bfbfbf" p-id="13133"></path><path d="M512 512m-80 0a80 80 0 1 0 160 0 80 80 0 1 0-160 0Z" fill="#bfbfbf" p-id="13134"></path><path d="M800 512m-80 0a80 80 0 1 0 160 0 80 80 0 1 0-160 0Z" fill="#bfbfbf" p-id="13135"></path></svg>
	</div>
	<Screenshot />
	<Greet />
	<Operate />
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
