<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { onMounted } from "vue";

function minimizeWindow() {
    appWindow.minimize();
}

function closeWindow() {
    appWindow.close();
}

onMounted(() => {
    setTimeout(() => {
        invoke("close_splashscreen");
    }, 2000);
})
</script>

<template>
    <div data-tauri-drag-region class="titlebar">
        <div class="title">Paradox Game Savefiles Management</div>
        <div>
            <div @click="minimizeWindow()" class="titlebar-button">
                <i class="iconfont icon-subtract"></i>
            </div>
            <div @click="closeWindow()" class="titlebar-button close-btn">
                <i class="iconfont icon-close1"></i>
            </div>
        </div>
    </div>
    <div class="content">
        <RouterView v-slot="{ Component }">
            <component :is="Component" />
        </RouterView>
    </div>
</template>

<style lang="less" scoped>
.title {
    margin-left: 10px;
}
.titlebar {
    height: 30px;
    background: #114514;
    user-select: none;
    display: flex;
    justify-content: flex-end;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    color: #cccccc;
    border-top-left-radius: 6px;
    border-top-right-radius: 6px;
    overflow: hidden;
}
.titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
}
.titlebar-button:hover {
    background: #193601;
}
.close-btn:hover {
    background: #c70404;
    color: #f4f7f3;
}
.titlebar>div:first-of-type {
    line-height: 30px;
    padding: 0 10;
    font-size: 14px;
    margin-right: auto;
}
.content {
    width: 100%;
    height: calc(100vh - 30px);
    margin-top: 5px;
    border-bottom-left-radius: 6px;
    border-bottom-right-radius: 6px;
    color: #f1eaea;
    background: #F5ECD7;
    font-size: 14px;
    display: flex;
    // justify-content: center;
    // align-items: center;
    overflow: hidden;
}
</style>