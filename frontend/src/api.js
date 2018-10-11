import axios from "axios";

const baseUrl = "/api/v1";

class API {
    static Login(email, password) {
        return axios.post(baseUrl + "/user/login", {
            email: email,
            password: password,
        });
    }
}

export default API;
