import VueRouter from "vue-router";

import Login from "./user/Login";
import Register from "./user/Register";

const routes = [
    {
        name: "user/login",
        path: "/user/login",
        component: Login,
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
