<template>
    <div>
        <create v-if="!team"></create>
        <profile v-if="team"></profile>
    </div>
</template>

<script>
import Vue from "vue";
import Component from "vue-class-component";
import API from "../api";

import Create from "./Create";
import Profile from "./Profile";

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
