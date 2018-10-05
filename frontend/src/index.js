import Vue from "vue";
import VueRouter from "vue-router";

import BootstrapVue from "bootstrap-vue";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

import App from "./App";
import router from "./routes";

Vue.use(BootstrapVue);
Vue.use(VueRouter);

document.addEventListener("DOMContentLoaded", function(event) { 
    new Vue({
        router,
        el: "#app",
        components: { App },
        render: h => h(App),
    });
});