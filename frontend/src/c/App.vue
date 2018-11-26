<template>
    <div>
        <navbar v-bind:loggedIn="loggedIn" v-bind:isAdmin="isAdmin"></navbar>
        <transition>
            <router-view></router-view>
        </transition>
    </div>
</template>

<script>
import Navbar from "./Navbar";

export default {
  components: { Navbar },
  data: () => ({
    loggedIn: false,
    isAdmin: false
  }),
  name: "App"
};
</script>

<style lang="scss" scoped>
</style>
