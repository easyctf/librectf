<template>
    <b-container id="create">
        <p>Looks like you don't have a team yet! Create a new team or join an existing one below:</p>
        <b-row>
            <b-col>
                <b-card title="Create New Team">
                    <b-form @submit.prevent="processForm">
                        <b-form-group id="emailGroup"
                            label="Team Name"
                            label-for="name">
                            <b-form-input id="name"
                                type="text"
                                v-model="name"
                                required
                                placeholder="Team Name">
                            </b-form-input>
                        </b-form-group>

                        <b-form-group class="text-center">
                            <b-button type="submit" class="col" variant="primary" :disabled="pending">Submit</b-button>
                        </b-form-group>
                    </b-form>
                </b-card>
            </b-col>
            <b-col>
                <b-card title="Join Existing Team">
                    <b-jumbotron>
                        <center>You need an invitation to join an existing team!</center>
                    </b-jumbotron>
                </b-card>
            </b-col>
        </b-row>
    </b-container>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import API from "../api";
import NProgress from "nprogress";

@Component
export default class Create extends Vue {
    name = ""
    pending = false

    processForm() {
        NProgress.start();
        this.pending = true;
        API.TeamCreate(
            this.name,
        ).then(() => {
            NProgress.done();
            this.pending = false;
            this.$router.push("/");
        });
    }
}
</script>

<style lang="scss" scoped>
    #create {
        margin-top: 28px;
    }
</style>
