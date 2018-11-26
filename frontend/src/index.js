import "babel-core/register";
import "babel-polyfill";

import Vue from "vue";
import Vuex from "vuex";
import VueRouter from "vue-router";

import BootstrapVue from "bootstrap-vue";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

import App from "./c/App";
import router from "./routes";
import store from "./store";

Vue.use(BootstrapVue);

document.addEventListener("DOMContentLoaded", function(event) { 
  new Vue({
    store,
    router,
    el: "#app",
    components: { App },
    render: h => h(App),
  });
});
