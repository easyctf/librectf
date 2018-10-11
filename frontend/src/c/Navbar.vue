<template>
    <b-navbar class="navbar" toggleable="md" type="dark" variant="dark">
        <div class="container">
            <b-navbar-toggle target="nav_collapse"></b-navbar-toggle>
            <b-navbar-brand :to="{ name: 'index' }">OpenCTF</b-navbar-brand>
            <b-collapse is-nav id="nav_collapse">
                <b-navbar-nav>
                    <b-nav-item href="#">Link</b-nav-item>
                    <b-nav-item href="#" disabled>Disabled</b-nav-item>
                </b-navbar-nav>
                    <b-navbar-nav class="ml-auto">
                        <template v-if="loggedIn">
                            <b-nav-item-dropdown right>
                                <template slot="button-content">
                                    <b>{{ username }}</b>
                                </template>
                                <b-dropdown-item href="/user/profile">Me</b-dropdown-item>
                                <b-dropdown-item @click="logout">Logout</b-dropdown-item>
                            </b-nav-item-dropdown>
                        </template>
                        <template v-else>
                            <b-nav-item :to="{ name: 'user/login' }">Login</b-nav-item>
                            <b-nav-item :to="{ name: 'user/register' }">Register</b-nav-item>
                        </template>
                    </b-navbar-nav>
            </b-collapse>
        </div>
    </b-navbar>
</template>

<script>
    export default {
        name: "Navbar",
        methods: {
            logout: function() {
                this.$store.dispatch("logout");
                this.$router.push("/");
            }
        },
        computed: {
            loggedIn() {
                return this.$store.getters.loggedIn;
            },
            username() {
                return this.$store.getters.username;
            }
        }
    }
</script>

<style lang="scss" scoped>
    .navbar {
        margin-bottom: 28px;
    }
</style>
