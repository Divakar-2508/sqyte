<script lang="ts">
    let commands: Array<string> = ["Hello"];
    let executeCommand: (cmd: string) => void;

    let input = "";
</script>

<div class="container"></div>

<style>
    .container {
        height: 120px;
        width: 750px;
        border-width: 2px 0px 0px 0px;
        border-style: solid;
        border-color: var(--stroke-primary);
    }
</style>
