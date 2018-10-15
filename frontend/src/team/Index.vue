<template>
    <div>
        <Profile v-if="team"></Profile>
        <Create v-else></Create>
    </div>
</template>

<script>
    import API from "../api";
    import NProgress from "nprogress";

    import Create from "./Create";
    import Profile from "./Profile";

    export default {
        name: "Profile",
        components: { Create, Profile },
        data: () => ({
            team: null,
        }),
        async created() {
            NProgress.start();
            let result = await API.TeamProfile();
            NProgress.done();
            this.team = result.data.team;
        }
    }
</script>
