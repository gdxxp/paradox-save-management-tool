<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref, onBeforeMount, watch, onBeforeUnmount } from "vue";
import { useRoute } from "vue-router";

import { FormItem, ListenParams } from "../common/interface/savefile";
import Notification from "../components/Notification.vue";

const route = useRoute();

let showNotification = ref(false);
let showSelectItems = ref(false);
let buttonLabel = ref('Start');
let replacedItem = ref<number>(-1);
let formItems = ref<FormItem[]>([]);
let notificationMessage = ref<string>();
let notificationType = ref<string>();
let listenedSavefiles = ref<ListenParams[]>([]);
let pollingInterval: any;

function showSuccess() {
    showNotification.value = true;
    notificationType.value = "success";
    notificationMessage.value = "Success";
    setTimeout(() => {
        showNotification.value = false;
    }, 1500);
}
function showError() {
    showNotification.value = true;
    notificationType.value = "error";
    notificationMessage.value = "Error";
    setTimeout(() => {
        showNotification.value = false;
    }, 1500);
}
function showCaution(message: string) {
    showNotification.value = true;
    notificationType.value = "caution";
    notificationMessage.value = message;
    setTimeout(() => {
        showNotification.value = false;
    }, 3000);
}

function backUpItem(index: number) {
    invoke("back_up_savefile", {back_up_item: formItems.value[index].location})
    .then(() => {
        getFormItems();
        showSuccess();
    }).catch((e) => {
        console.log(e);
        showCaution("Please confirm whether the file is in the save games folder");
    })
}

function deleteItem(index: number) {
    if (buttonLabel.value === "Stop") {
        showCaution("Please stop first");
        return;
    }
    invoke("delete_savefile", {delete_item: formItems.value[index].location})
    .then((result) => {
        console.log(result);
        getFormItems();
        showSuccess();
        formItems.value.splice(index, 1);
    }).catch((e) => {
        console.log(e);
        showError();
    });
}

function replaceItem(index: number) {
    if (buttonLabel.value === "Stop") {
        showCaution("Please stop first");
        return;
    }
    invoke("replace_savefile", {replaced_item: formItems.value[replacedItem.value].location, replace_item: formItems.value[index].location})
    .then(() => {
        getFormItems();
        showSuccess();
    }).catch(
        (e) => {
            console.log(e);
            showError()
        });
    replacedItem.value = -1;
    showSelectItems.value = false;
}

function cancelReplace() {
    replacedItem.value = -1;
    showSelectItems.value = false;
}
function chooseReplacedItem(index: number) {
    replacedItem.value = index;
    showSelectItems.value = true;
}

function openWindowsFolder() {
    const gameName = route.name;
    invoke("open_file_explorer",{game_name: gameName});
}

function checkCheckbox() {
    listenedSavefiles.value = [];
    formItems.value.forEach((element) => {
        if (element.isChecked) {
            listenedSavefiles.value.push(element);
        }
    });
}

function listening() {
    checkCheckbox();
    if (buttonLabel.value === 'Start' && listenedSavefiles.value.length > 0) {
        buttonLabel.value = 'Stop';
    }
    else if (buttonLabel.value === 'Stop') {
        buttonLabel.value = 'Start';
    }
    else {
        showCaution("Please select the savefile that you want to listen to");
    }
}

function startListen() {
    if (!pollingInterval) {
        pollingInterval = setInterval(() => {
            invoke("listen_savefiles", {savefiles: listenedSavefiles.value});
            getFormItems();
        }, 2000);
    }
}

function stopListen() {
    if (pollingInterval) {
        clearInterval(pollingInterval);
        pollingInterval = null;
    }
}

function getFormItems() {
    const gameName = route.name;
    invoke("load_savefiles", {game_name: gameName}).then((result) => {
        checkCheckbox();
        formItems.value = result as FormItem[]
        formItems.value.forEach((formItem) => {
            listenedSavefiles.value.forEach((listenedItem) => {
                if (formItem.location === listenedItem.location) {
                    formItem.isChecked = true;
                }
            })
        });
    }).catch((e) => console.error(e));
}

watch(buttonLabel, (newValue, oldValue) => {
    if (oldValue === "Start") {
        startListen();
    }
    else {
        stopListen();
    }
    console.log(`Status changed from ${oldValue} to ${newValue}`);
});

onBeforeMount(() => {
    getFormItems();
})

onBeforeUnmount(() => {
    stopListen();
})
</script>

<template>
    <div class="content">
        <div class="left">
            <div class="back">
                <RouterLink to="/choice">
                <a>Back</a>
                </RouterLink>
            </div>
            <div @click="openWindowsFolder()">
                <a>Open the folder</a>
            </div>
        </div>
        <div class="middle">
            <div class="form-container">
                <div v-for="(item, index) in formItems" :key="index" :class="{'form-item': !(item.isBackups), 'backups-item': item.isBackups}">
                    <div class="checkbox">
                        <input v-if="!item.isBackups" v-model="item.isChecked" type="checkbox" />
                    </div>
                    <h3>{{ item.name }}</h3>
                    <div class="time">{{ item.updateTime }}</div>
                    <div class="operation">
                        <span class="backup" v-if="!showSelectItems&&!(item.isBackups)" @click="backUpItem(index)">Backup</span>
                        <span class="replace" v-if="!showSelectItems" @click="chooseReplacedItem(index)">Replace</span>
                        <span class="delete" v-if="!showSelectItems" @click="deleteItem(index)">Delete</span>
                        <span class="select-cancel" v-if="showSelectItems&&(replacedItem!==index)" @click="replaceItem(index)">Select</span>
                        <span class="select-cancel" v-else-if="showSelectItems&&(replacedItem===index)" @click="cancelReplace">Cancel</span>
                    </div>
                </div>
            </div>
            <div class="start-button">
                <button :class="buttonLabel" @click="listening">
                    {{ buttonLabel }}
                </button>
            </div>
        </div>
        <Notification v-if="showNotification" :type="notificationType" :message="notificationMessage"></Notification>
    </div>
</template>

<style lang="less" scoped>
.content {
  display: flex;
}
.left {
    margin-top: 35px;
    margin-left: 10px;
    .back {
        margin-bottom: 20px;
    }
}
.middle {
    margin-top: 2%;
    margin-left: 3%;
    width: 80%;
    height: 90%;
    .form-container {  
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    background-color: #ebe2cd;
    border-radius: 10px;
    overflow-y: auto;
    overflow-x: hidden;
        .form-item {
        margin: 5px;
        padding: 10px;
        width: 99%;
        background-color: #c2baa6;
        display: flex;
        border-radius: 10px;
        }
        .backups-item {
        margin: 5px;
        padding: 10px;
        width: 99%;
        background-color: #8FBF9F;
        display: flex;
        align-items: center;
        border-radius: 10px;
        }
    }
}
.checkbox {
    width: 20px;
    margin-top: 4px;
}
.time {
    margin-left: 40%;
    color: black;
}
.operation {
    margin-left: auto;
}
.replace {
    margin-left: 20px;
}
.delete {
    margin-left: 20px;
}
.select-cancel {
    margin-left: auto;
}
.start-button {
    margin-left: 90%;
    margin-top: 6px;
}
.Start {
    width: 65px;
    height: 30px;
    border: none;
    border-radius: 6px;
    background-color: #68a67d;
}
.Stop {
    width: 65px;
    height: 30px;
    border: none;
    border-radius: 6px;
    background-color: #e03f3f;
}
button:hover {
    cursor: pointer;
}
span {
    color: black;
}
span:hover {
    color: #c80909;
    cursor: pointer;
}
h3 {
    width: 20%;
    color: black;
}
h4 {
    color: black;
}
a {
    color: black;
    text-decoration: none;
}
a:hover {
    color: #126550;
    cursor: pointer;
    text-decoration: none;
}
</style>