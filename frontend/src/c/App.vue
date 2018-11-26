<template>
    <div>
        <navbar></navbar>
        <transition>
            <router-view></router-view>
        </transition>
    </div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

import Navbar from "./Navbar";

@Component({
    components: { Navbar }
})
export default class App extends Vue {
}
</script>

<style lang="scss" scoped>
</style>
