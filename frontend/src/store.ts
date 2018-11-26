import Vue from "vue";
import Vuex from "vuex";
Vue.use(Vuex);

import jwtDecode from "jwt-decode";

import API from "./api";

const store = new Vuex.Store({
    state: {
        session: (x => {
            try {
                return x ? jwtDecode(x) : undefined;
            } catch (_) {
                return undefined;
            }
        })(localStorage.getItem("token")),
        pending: false,
    },
    mutations: {
        ["LOGIN_ATTEMPT"](state) {
            state.pending = true;
        },
        ["LOGIN_SUCCESS"](state, session) {
            state.session = session;
            state.pending = false;
        },
        ["LOGOUT"](state) {
            state.session = undefined;
        }
    },
    actions: {
        login({ commit }, credentials) {
            commit("LOGIN_ATTEMPT");
            return API.UserLogin(credentials.user, credentials.password).then((result) => {
                localStorage.setItem("token", result.data);
                let session = jwtDecode(result.data);
                commit("LOGIN_SUCCESS", session);
            });
        },
        logout({ commit }) {
            localStorage.removeItem("token");
            commit("LOGOUT");
        }
    },
    getters: {
        session: state => {
            return state.session;
        },
        username: state => {
            if (!state.session) {
                return false;
            }
            return state.session.username;
        }
    }
});

export default store;
