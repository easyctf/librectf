import Vue from "vue";
import VueRouter from "vue-router";
Vue.use(VueRouter);

import Home from "./c/Home";
import Login from "./user/Login";
import Profile from "./user/Profile";
import Register from "./user/Register";

const routes = [
    {
        name: "index",
        path: "/",
        component: Home,
    },
    {
        name: "user/login",
        path: "/user/login",
        component: Login,
    },
    {
        name: "user/profile",
        path: "/user/profile",
        component: Profile,
    },
    {
        name: "user/register",
        path: "/user/register",
        component: Register,
    }
];

const router = new VueRouter({
    routes,
    mode: "history",
});

export default router;
