import axios from "axios";

const baseUrl = "/api/v1";

const alwaysReject = new Promise((_, reject) => reject());

class API {
    static jwtGet(url: string): Promise<any> {
        let token = localStorage.getItem("token");
        if (!token) {
            return alwaysReject;
        }
        return axios
            .get(url, { headers: { Authorization: `Token ${token}` }})
            .catch((err) => console.dir(err));
    }
    static jwtPost(url: string, data: object): Promise<any> {
        let token = localStorage.getItem("token");
        if (!token) {
            return alwaysReject;
        }
        return axios
            .post(url, data, { headers: { Authorization: `Token ${token}` }})
            .catch((err) => console.dir(err));
    }

    static ChalList() {
        return API.jwtGet(baseUrl + "/chal/list");
    }

    static ChalSubmit(id: string, flag: string) {
        return API.jwtPost(baseUrl + "/chal/submit", {
            id,
            flag,
        });
    }

    static UserLogin(user: string, password: string) {
        return axios.post(baseUrl + "/user/login", {
            user,
            password,
        });
    }

    static UserRegister(email: string, username: string, password: string) {
        return axios.post(baseUrl + "/user/register", {
            email,
            username,
            password,
        });
    }

    static TeamCreate(name: number) {
        return API.jwtPost(baseUrl + "/team/create", {
            name,
        });
    }

    static TeamProfile(id: number) {
        if (id === undefined) {
            return API.jwtGet(baseUrl + "/team/me");
        } else {
            return API.jwtGet(baseUrl + "/team/profile/" + id);
        }
    }
}

export default API;
