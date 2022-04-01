<script lang="ts">
    import { Table, Tooltip } from 'sveltestrap';

    export let tableData: {
        header: string[],
        rows: any,
    };
</script>

<Table hover>
    <thead>
        <tr>
            {#each tableData.header as col}
                <th>{col}</th>
            {/each}
        </tr>
    </thead>
    <tbody>
        {#each tableData.rows as row}
            {@const uuid = Math.random().toString(36).slice(-6)}
            <tr id={`row-${uuid}`}>
                {#each row as col}
                    <td>{ col }</td>
                {/each}
            </tr>
            <Tooltip target={`row-${uuid}`} placement="bottom">{uuid}</Tooltip>
        {/each}
    </tbody>
</Table>