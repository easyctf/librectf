import "babel-core/register";
import "babel-polyfill";

import Vue from "vue";
import Toaster from "v-toaster";

import App from "./c/App";
import router from "./routes";
import store from "./store";

// CSS
import BootstrapVue from "bootstrap-vue";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";
import "v-toaster/dist/v-toaster.css";

Vue.use(BootstrapVue);
Vue.use(Toaster, {timeout: 5000});

document.addEventListener("DOMContentLoaded", function(_event) { 
    new Vue({
        store,
        router,
        el: "#app",
        components: { App },
        render: h => h(App),
    });
});
