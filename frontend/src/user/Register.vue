<template>
    <section class="h-100">
        <div class="container h-100">
            <div class="row justify-content-md-center h-100">
                <b-card title="Register">
                    <b-form @submit.prevent="processForm">
                        <b-form-group id="emailGroup"
                            label="Email Address"
                            label-for="email">
                            <b-form-input id="email"
                                type="email"
                                required
                                placeholder="Email Address">
                            </b-form-input>
                        </b-form-group>
                        <b-form-group id="usernameGroup"
                            label="Username"
                            label-for="username">
                            <b-form-input id="username"
                                type="text"
                                required
                                placeholder="Username">
                            </b-form-input>
                        </b-form-group>
                        <b-form-group id="passwordGroup"
                            label="Password"
                            label-for="password">
                            <b-form-input id="password"
                                type="password"
                                required
                                placeholder="Password">
                            </b-form-input>
                        </b-form-group>

                        <b-form-group class="text-center">
                            <b-button type="submit" class="col" variant="primary">Submit</b-button>
                        </b-form-group>

                        <div class="margin-top20 text-center">
                            Have an account? <router-link :to="{ name: 'user/login' }">Login</router-link>
                        </div>
                    </b-form>
                </b-card>
            </div>
        </div>
    </section>
</template>

<script>
    export default {
        name: "Register",
        data: () => ({
            pending: false,
        }),
        methods: {
            processForm: function() {
                this.pending = true;
            }
        }
    }
</script>

<style lang="scss" scoped>
    .card-body {
        min-width: 450px;
    }
</style>
