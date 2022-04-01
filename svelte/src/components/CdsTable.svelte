
<!--
    TODO: code mover (different interface for production control?)
    TODO: parts on nest tooltips
    TODO: program link
-->


<script lang="ts">
    import { Popover, Table } from 'sveltestrap';

    export let tableData: {
        header: string[],
        rows: any,
    };

    let openPartListId = null;

    const partTableData = {
        header: [ "Part", "Qty" ],
        rows: [
            { part: "x1a", qty: 5 },
            { part: "x1b", qty: 2 },
            { part: "m2d", qty: 16 },
        ]
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
            {@const rowId = `row-${Math.random().toString(36).slice(-6)}`}
            
            <tr id={ rowId }>
                {#each row as col}
                    <td>{ col }</td>
                {/each}
            </tr>
            <Popover trigger="hover" placement="bottom" target={ rowId } title="Part List">
                <Table>
                    <thead>
                        <tr>
                            {#each partTableData.header as hr}
                                <th>{ hr }</th>
                            {/each}
                        </tr>
                    </thead>
                    <tbody>
                        {#each partTableData.rows as row}
                            <tr>
                                <td>{ row.part }</td>
                                <td>{ row.qty }</td>
                            </tr>
                        {/each}
                    </tbody>
                </Table>
            </Popover>
        {/each}
    </tbody>
</Table>