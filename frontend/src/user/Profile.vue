<template>
    <div class="container">
        profile of {{ session.username }}
    </div>
</template>

<script>
    export default {
        name: "Profile",
        computed: {
            session() {
                return this.$store.getters.session;
            },
        }
    }
</script>