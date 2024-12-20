<script lang="ts">
    import Sidebar from "../components/Sidebar.svelte";
    import TerminalBar from "../components/TerminalBar.svelte";
    import MainSpace from "../components/MainSpace.svelte";
    import ToolBar from "../components/ToolBar.svelte";

    let activeTable: Table | null = null;
    let databaseName: string = "baka";
</script>

<div class="container">
    <Sidebar {activeTable} {databaseName} />
    <div class="workspace">
        <ToolBar />
        <MainSpace />
        <TerminalBar />
    </div>
</div>

<style>
    .container {
        height: 100vh;
        width: 100wh;
        background-color: var(--bg-primary);
        display: flex;
        flex-direction: row;
    }

    .workspace {
        display: flex;
        flex-direction: column;
        flex-grow: 1;
    }
</style>
