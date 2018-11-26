<template>
    <div>
        <b-jumbotron>
            <b-container>
                <h1>Login</h1>
            </b-container>
        </b-jumbotron>
        <section class="h-100">
            <div class="container h-100">
                <div class="row justify-content-md-center h-100">
                    <b-card>
                        <b-form @submit.prevent="processForm">
                            <b-form-group id="emailGroup"
                                label="Email or Username"
                                label-for="user">
                                <b-form-input id="user"
                                    type="text"
                                    v-model="user"
                                    required
                                    placeholder="Email or Username">
                                </b-form-input>
                            </b-form-group>
                            <b-form-group id="passwordGroup"
                                label="Password"
                                label-for="password">
                                <b-form-input id="password"
                                    type="password"
                                    v-model="password"
                                    required
                                    placeholder="Password">
                                </b-form-input>
                            </b-form-group>

                            <b-form-group class="text-center">
                                <b-button type="submit" class="col" variant="primary" :disabled="pending">Submit</b-button>
                            </b-form-group>

                            <div class="margin-top20 text-center">
                                Need an account? <router-link :to="{ name: 'user/register' }">Register</router-link>
                            </div>
                        </b-form>
                    </b-card>
                </div>
            </div>
        </section>
    </div>
</template>

<script>
import NProgress from "nprogress";

export default {
    name: "Login",
    data: () => ({
        user: "",
        password: "",
        pending: false,
    }),
    methods: {
        processForm: function() {
            NProgress.start();
            this.pending = true;
            this.$store.dispatch("login", {
                user: this.user,
                password: this.password,
            }).then(() => {
                NProgress.done();
                this.pending = false;
                this.$router.push("/");
            });
        }
    }
};
</script>

<style lang="scss" scoped>
    .card-body {
        min-width: 450px;
    }
</style>
