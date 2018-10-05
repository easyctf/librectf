<template>
    <div>
        <b-navbar toggleable="md" type="dark" variant="dark">
            <div class="container">
                <b-navbar-toggle target="nav_collapse"></b-navbar-toggle>
                <b-navbar-brand href="/">OpenCTF</b-navbar-brand>
                <b-collapse is-nav id="nav_collapse">
                    <b-navbar-nav>
                        <b-nav-item href="#">Link</b-nav-item>
                        <b-nav-item href="#" disabled>Disabled</b-nav-item>
                    </b-navbar-nav>
                </b-collapse>
            </div>
        </b-navbar>
        <router-view></router-view>
    </div>
</template>

<script>
    export default {
        name: "App",
    }
</script>

<style lang="scss">
</style>
