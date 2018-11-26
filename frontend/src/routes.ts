import Vue from "vue";
import VueRouter from "vue-router";
import * as NProgress from "nprogress";
Vue.use(VueRouter);

import Home from "./c/Home.vue";
import Scoreboard from "./c/Scoreboard.vue";
import ChalList from "./chal/List.vue";
import TeamIndex from "./team/Index.vue";
import TeamProfile from "./team/Profile.vue";
import UserLogin from "./user/Login.vue";
import UserRegister from "./user/Register.vue";
import UserSettings from "./user/Settings.vue";

const routes = [
    {
        name: "index",
        path: "/",
        component: Home,
    },
    {
        name: "scoreboard",
        path: "/scoreboard",
        component: Scoreboard,
    },
    {
        name: "chal/list",
        path: "/chal/list",
        component: ChalList,
    },
    {
        name: "team",
        path: "/team",
        component: TeamIndex,
    },
    {
        name: "team/profile",
        path: "/team/profile",
        component: TeamProfile,
    },
    {
        name: "user/login",
        path: "/user/login",
        component: UserLogin,
    },
    {
        name: "user/register",
        path: "/user/register",
        component: UserRegister,
    },
    {
        name: "user/settings",
        path: "/user/settings",
        component: UserSettings,
    },
];

const router = new VueRouter({
    routes,
    mode: "history",
    beforeEach: (to, _from, next) => {
        document.title = to.meta.title ? (to.meta.title + " - LibreCTF") : "LibreCTF";
        next();
    }
});

router.beforeResolve((to, _from, next) => {
    if (to.name) {
        NProgress.start();
    }
    next();
});

router.afterEach((_to, _from) => {
    NProgress.done();
});

export default router;
