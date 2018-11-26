<template>
    <b-navbar-nav class="ml-auto">
        <template v-if="session">
            <b-nav-item v-if="hasTeam" :to="{ name: 'chal/list' }">Challenges</b-nav-item>
            <b-nav-item-dropdown right v-if="session.admin">
                <template slot="button-content">
                    Admin
                </template>
                <b-dropdown-item :to="{ name: 'admin/overview' }">Overview</b-dropdown-item>
            </b-nav-item-dropdown>
            <b-nav-item :to="{ name: 'team' }">Team</b-nav-item>
            <b-nav-item :to="{ name: 'user/settings' }">Settings</b-nav-item>
            <b-nav-item @click="logout">Logout</b-nav-item>
        </template>
        <template v-else>
            <b-nav-item :to="{ name: 'user/login' }">Login</b-nav-item>
            <b-nav-item :to="{ name: 'user/register' }">Register</b-nav-item>
        </template>
    </b-navbar-nav>
</template>

<script>
import Vue from "vue";
import Component from "vue-class-component";

@Component
export default class User extends Vue {
    get session() {
        return this.$store.getters.session;
    }
    get hasTeam() {
        return this.$store.getters.hasTeam;
    }
    logout() {
        this.$store.dispatch("logout");
        this.$toaster.success("Logged out.");
        this.$router.push("/");
    }
}
</script>

<style scoped>
</style>
