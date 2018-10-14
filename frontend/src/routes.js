import Vue from "vue";
import VueRouter from "vue-router";
Vue.use(VueRouter);

import Home from "./c/Home";
import TeamCreate from "./team/Create";
import TeamProfile from "./team/Profile";
import UserLogin from "./user/Login";
import UserRegister from "./user/Register";

const routes = [
    {
        name: "index",
        path: "/",
        component: Home,
    },
    {
        name: "team/create",
        path: "/team/create",
        title: "Create Team",
        component: TeamCreate,
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
        component: UserRegister,
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

export default router;
