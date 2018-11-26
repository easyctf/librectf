<template>
    <div>
        <b-jumbotron>
            <b-container>
                <h1>Challenges</h1>
            </b-container>
        </b-jumbotron>
        <b-container>
            <div v-for="(chal, i) in challenges">
                <b-card no-body role="tab">
                    <b-card-header v-b-toggle="'chalCollapse' + i">
                        {{ chal.title }} ({{ chal.value }} point{{ chal.value == 1 ? "" : "s" }})
                    </b-card-header>
                    <b-collapse :id="'chalCollapse' + i" visible>
                        <b-card-body>
                            <p v-html="chal.description"></p>
                            <b-form>
                                <b-input-group>
                                    <b-form-input :id="'flagSubmit' + i"
                                        type="text"
                                        required
                                        placeholder="flag{...}">
                                    </b-form-input>
                                    <b-input-group-append>
                                        <b-button type="submit" variant="primary">Submit</b-button>
                                    </b-input-group-append>
                                </b-input-group>
                            </b-form>
                        </b-card-body>
                    </b-collapse>
                </b-card>
                <p></p>
            </div>
        </b-container>
    </div>
</template>

<script>
import API from "../api";

export default {
    name: "List",
    data: () => ({
        challenges: []
    }),
    async created() {
        let result = await API.ChalList();
        this.challenges = result.data;
    }
};
</script>
