import VueRouter from "vue-router";

import Login from "./user/Login";

const routes = [
    {
        name: "user/login",
        path: "/user/login",
        component: Login,
    }
];

const router = new VueRouter({
    routes,
    mode: "history",
});

export default router;
