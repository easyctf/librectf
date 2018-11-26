import "babel-core/register";
import "babel-polyfill";

import Vue from "vue";
import Snotify from "vue-snotify";

import App from "./c/App.vue";
import router from "./routes";
import store from "./store";

// CSS
import BootstrapVue from "bootstrap-vue";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";
import "v-toaster/dist/v-toaster.css";

Vue.use(BootstrapVue);
Vue.use(Snotify);

document.addEventListener("DOMContentLoaded", function(_) { 
    new Vue({
        store,
        router,
        components: { App },
        render: h => h(App),
    }).$mount("#app");
});
