<template>
    <div>
        <Create v-if="!team"></Create>
    </div>
</template>

<script>
    import API from "../api";
    import NProgress from "nprogress";

    import Create from "./Create";
    import Profile from "./Profile";

    export default {
        name: "Index",
        components: { Create, Profile },
        data: () => ({
            team: true,
        }),
        async created() {
            NProgress.start();
            let result = await API.TeamProfile();
            NProgress.done();
            if (result.data && result.data.team)
                this.$router.push("/team/profile");
            else
                this.team = false;
        }
    }
</script>
