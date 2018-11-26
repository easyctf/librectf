<template>
    <b-jumbotron>
        <b-container>
            <h1 class="display-3">LibreCTF</h1>
            <p class="lead">Open-source CTF framework.</p>
        </b-container>
    </b-jumbotron>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

@Component
export default class Home extends Vue {
}
</script>

<style lang="scss" scoped>
#home {
  margin-top: 28px;
}
</style>
