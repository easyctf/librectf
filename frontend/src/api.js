import axios from "axios";

const baseUrl = "/api/v1";

class API {
    static UserLogin(email, password) {
        return axios.post(baseUrl + "/user/login", {
            email,
            password,
        });
    }

    static UserRegister(email, username, password) {
        return axios.post(baseUrl + "/user/register", {
            email,
            username,
            password,
        });
    }

    static TeamCreate(name) {
        let token = localStorage.getItem("token");
        if (!token) return new Promise((_, reject) => reject());
        return axios.post(baseUrl + "/team/create", {
            name,
        });
    }
}

export default API;
