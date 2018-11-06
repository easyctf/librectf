<template>
    <b-navbar class="navbar" toggleable="md" type="dark" variant="dark">
        <b-container>
            <b-navbar-toggle target="nav_collapse"></b-navbar-toggle>
            <b-navbar-brand :to="{ name: 'index' }">LibreCTF</b-navbar-brand>
            <b-collapse is-nav id="nav_collapse">
                <b-navbar-nav>
                    <b-nav-item :to="{ name: 'scoreboard' }">Scoreboard</b-nav-item>
                </b-navbar-nav>
                    <b-navbar-nav class="ml-auto">
                        <template v-if="session">
                            <b-nav-item :to="{ name: 'chal/list' }">Challenges</b-nav-item>
                            <b-nav-item-dropdown right v-if="session.admin">
                                <template slot="button-content">
                                    Admin
                                </template>
                                <b-dropdown-item :to="{ name: 'admin/overview' }">Overview</b-dropdown-item>
                            </b-nav-item-dropdown>
                            <b-nav-item-dropdown right>
                                <template slot="button-content">
                                    {{ username }}
                                </template>
                                <b-dropdown-item :to="{ name: 'team' }">My Team</b-dropdown-item>
                                <b-dropdown-item :to="{ name: 'user/settings' }">Settings</b-dropdown-item>
                                <b-dropdown-divider></b-dropdown-divider>
                                <b-dropdown-item @click="logout">Logout</b-dropdown-item>
                            </b-nav-item-dropdown>
                        </template>
                        <template v-else>
                            <b-nav-item :to="{ name: 'user/login' }">Login</b-nav-item>
                            <b-nav-item :to="{ name: 'user/register' }">Register</b-nav-item>
                        </template>
                    </b-navbar-nav>
            </b-collapse>
        </b-container>
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
            session() {
                return this.$store.getters.session;
            },
            username() {
                return this.$store.getters.username;
            }
        }
    }
</script>

<style lang="scss" scoped>
</style>
