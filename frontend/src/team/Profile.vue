<template>
    <div>
        <div v-if="team">
            <!-- Jumbo header -->
            <b-jumbotron>
                <b-container>
                     <b-dropdown right text="Actions" class="float-right">
                        <b-dropdown-item>Edit Team Information</b-dropdown-item>
                        <b-dropdown-item>Manage Members</b-dropdown-item>
                    </b-dropdown>

                    <h1>{{ team.name }}</h1>
                    <p>{{ team.affiliation || "No affiliation" }}</p>
                </b-container>
            </b-jumbotron>

            <!-- Profile table -->
            <b-container>
                <b-row>
                    <b-col>
                        <!-- Team members -->
                        <b-card header="Team Members"
                            header-tag="header"
                            no-body>
                            <b-list-group>
                                <b-list-group-item>Cras justo odio</b-list-group-item>
                                <b-list-group-item>Dapibus ac facilisis in</b-list-group-item>
                                <b-list-group-item>Vestibulum at eros</b-list-group-item>
                            </b-list-group>
                        </b-card>
                    </b-col>
                    <b-col cols="8">
                        <!-- Statistics -->
                        <b-card no-body>
                            <b-tabs card>
                                <b-tab title="Overview" active>
                                    <h2>Recent Activity</h2>
                                </b-tab>
                                <b-tab title="Solves">
                                    I'm the second tab content
                                </b-tab>
                            </b-tabs>
                        </b-card>
                    </b-col>
                </b-row>
            </b-container>
        </div>

        <div v-else>
            <b-container>
                <p>Not found.</p>
            </b-container>
        </div>
    </div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import API from "../api";

@Component
export default class Profile extends Vue {
    team = null

    async created() {
        let result = await API.TeamProfile(0);
        if (result.data === null) {
            this.$router.push("/team/create");
        } else {
            this.team = result.data.team;
        }
    }
}
</script>
