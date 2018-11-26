<template>
    <b-navbar class="navbar" toggleable="md" type="dark" variant="dark">
        <b-container>
            <b-navbar-toggle target="nav_collapse"></b-navbar-toggle>
            <b-navbar-brand :to="{ name: 'index' }">LibreCTF</b-navbar-brand>
            <b-collapse is-nav id="nav_collapse">
                <b-navbar-nav>
                    <b-nav-item :to="{ name: 'scoreboard' }">Scoreboard</b-nav-item>
                </b-navbar-nav>
                <user></user>
            </b-collapse>
        </b-container>
    </b-navbar>
</template>

<script>
import User from "./User";

export default {
  name: "Navbar",
  components: { User },
  computed: {
    session() {
      return this.$store.getters.session;
    },
    username() {
      return this.$store.getters.username;
    }
  }
};
</script>

<style lang="scss" scoped>
</style>
