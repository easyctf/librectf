<template>
    <div>
        <!-- Jumbo header -->
        <b-jumbotron>
            <b-container>
                 <b-dropdown right text="Dropdown Button" class="float-right">
                    <b-dropdown-item>First Action</b-dropdown-item>
                    <b-dropdown-item>Second Action</b-dropdown-item>
                    <b-dropdown-item>Third Action</b-dropdown-item>
                    <b-dropdown-divider></b-dropdown-divider>
                    <b-dropdown-item>Something else here...</b-dropdown-item>
                    <b-dropdown-item disabled>Disabled action</b-dropdown-item>
                </b-dropdown>

                <h1>My Team Name</h1>
                <p>My Institution</p>
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
                        <b-list-group flush>
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
</template>

<script>
    export default {
        name: "Profile",
    }
</script>
