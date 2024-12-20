<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    export let activeTable: Table | null;
    export let databaseName: string;

    let tables: Array<Table> = [];
</script>

<div class="sidebar"></div>

<style>
    .sidebar {
        width: 250px;
        height: 600px;
        border-width: 0px 2px 0px 0px;
        border-style: solid;
        border-color: var(--stroke-primary);
    }
</style>
