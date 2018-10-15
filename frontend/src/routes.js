import Vue from "vue";
import VueRouter from "vue-router";
Vue.use(VueRouter);

import NProgress from "nprogress";

import Home from "./c/Home";
import Scoreboard from "./c/Scoreboard";
import Team from "./team/Index";
import UserLogin from "./user/Login";
import UserRegister from "./user/Register";
import UserSettings from "./user/Settings";

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
        name: "team",
        path: "/team",
        component: Team,
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
    beforeEach: (to, from, next) => {
        document.title = to.meta.title ? (to.meta.title + " - OpenCTF") : "OpenCTF";
        next();
    }
});

router.beforeResolve((to, from, next) => {
    if (to.name) {
        NProgress.start();
    }
    next();
})

router.afterEach((to, from) => {
    NProgress.done()
});

export default router;
