<template>
    <section class="h-100">
        <div class="container h-100">
            <div class="row justify-content-md-center h-100">
                <b-card title="Login">
                    <b-form @submit.prevent="processForm">
                        <b-form-group id="emailGroup"
                            label="Email Address"
                            label-for="email">
                            <b-form-input id="email"
                                type="email"
                                v-model="email"
                                required
                                placeholder="Email Address">
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
                            <b-button type="submit" class="col" variant="primary">Submit</b-button>
                        </b-form-group>

                        <div class="margin-top20 text-center">
                            Need an account? <router-link :to="{ name: 'user/register' }">Register</router-link>
                        </div>
                    </b-form>
                </b-card>
            </div>
        </div>
    </section>
</template>

<script>
    import API from "../api";
    export default {
        name: "Login",
        data: () => ({
            email: "",
            password: "",
        }),
        methods: {
            processForm: function() {
                console.log("processing.", this);
                this.$store.dispatch("login", {
                    email: this.email,
                    password: this.password,
                }).then(() => {
                    this.$router.push("/");
                });
            }
        }
    }
</script>

<style lang="scss" scoped>
    .card-body {
        min-width: 450px;
    }
</style>
