<script setup lang="ts">
defineProps({
    type: String,
    message: String,
});
</script>

<template>
    <teleport to="body">
        <div :class="type" class="notification">
            <span>{{ message }}</span>
        </div>
    </teleport>
</template>

<style lang="less" scoped>
.notification {
  position: fixed;
  bottom: 40px;
  left: 20px;
  padding: 10px;
  border-radius: 6px;
  color: #fff;
  font-weight: medium;
}

.success {
  background-color: #4caf4fa2;
}

.error {
  background-color: #f44336a4;
}
.caution {
    background-color: #afad4ca2
}
</style>