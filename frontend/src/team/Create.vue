<template>
    <div>
        <b-jumbotron>
            <b-container>
                <h1>Create a Team</h1>
            </b-container>
        </b-jumbotron>
        <section class="h-100">
            <div class="container h-100">
                <div class="row justify-content-md-center h-100">
                    <b-card>
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
                </div>
            </div>
        </section>
    </div>
</template>

<script>
    export default {
        name: "Create",
        data: () => ({
            name: "",
            pending: false,
        })
    }
</script>
