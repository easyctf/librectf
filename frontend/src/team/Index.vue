<template>
    <div>
        <create v-if="!team"></create>
        <profile v-if="team"></profile>
    </div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import API from "../api";

import Create from "./Create.vue";
import Profile from "./Profile.vue";

@Component({
    components: { Create, Profile }
})
export default class Index extends Vue {
    team = true
    async created() {
        let result = await API.TeamProfile();
        console.log(result);
    }
}
</script>
