<template>
    <div>
        <navbar></navbar>
        <transition>
            <router-view></router-view>
        </transition>
    </div>
</template>

<script>
    import Navbar from "./Navbar";

    export default {
        components: { Navbar },
        name: "App",
    }
</script>

<style lang="scss" scoped>
</style>
