<script setup lang="ts">

</script>

<template>
    <div class="content">
        <div class="grid">
            <router-link to="/ck3">
            <img src="../assets/cover/ck3.png" alt="ck3" />
            </router-link>
            <router-link to="/eu4">
                <img src="../assets/cover/eu4.png" alt="eu4" />
            </router-link>
            <router-link to="/hoi4">
                <img src="../assets/cover/hoi4.png" alt="hoi4" />
            </router-link>
            <router-link to="/stellaris">
                <img src="../assets/cover/stellaris.png" alt="stellaris" />
            </router-link>
            <router-link to="/v3">
                <img src="../assets/cover/v3.png" alt="v3" />
            </router-link>
            <router-link to="/ck2">
                <img src="../assets/cover/ck2.png" alt="ck2" />
            </router-link>
        </div>
    </div>
</template>

<style lang="less" scoped>
.content{
    justify-content: center;
    align-items: center;
}
.grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-gap: 100px;
    justify-items: center;
    align-items: center;
}
img {
    width: 342px;
    height: 117px;
    cursor: pointer;
    border-radius: 10px;
}
img:hover {
    transform: scale(1.3);
}
</style>