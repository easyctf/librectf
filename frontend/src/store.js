import Vue from "vue";
import Vuex from "vuex";
Vue.use(Vuex);

import API from "./api";

const store = new Vuex.Store({
    state: {
        loggedIn: !!localStorage.getItem("token"),
        pending: false,
    },
    mutations: {
        ["LOGIN_ATTEMPT"] (state) {
            state.pending = true;
        },
        ["LOGIN_SUCCESS"] (state) {
            state.loggedIn = true;
            state.pending = false;
        },
        ["LOGOUT"] (state) {
            state.loggedIn = false;
        }
    },
    actions: {
        login({ commit }, credentials) {
            commit("LOGIN_ATTEMPT");
            return API.Login(credentials.email, credentials.password).then((data) => {
                console.log(data);
            });
        }
    }
});

export default store;
