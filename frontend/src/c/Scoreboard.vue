<template>
    <div>
        <b-jumbotron>
            <b-container>
                <h1>Scoreboard</h1>
            </b-container>
        </b-jumbotron>
        <b-container>
            <p class="text-center" v-if="!scores.length">No scoring teams yet!</p>
            <b-table striped hover :items="scores"></b-table>
        </b-container>
    </div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

interface Score {
    rank: number;
    team: string;
    affiliation: string;
    points: number;
}

@Component
export default class Scoreboard extends Vue {
    mounted() {
    }
    get scores(): Score[] {
        return [];
    }
}
</script>
