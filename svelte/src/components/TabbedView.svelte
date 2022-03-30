
<!--
    TODO: add filters:
        - machine
        - girder group (webs/flanges)
    TODO: code mover (different interface for production control?)
    TODO: parts on nest tooltips
    TODO: program link
-->

<script>
    import { TabContent, TabPane, Table } from 'sveltestrap';

    export let data;
</script>

<TabContent>
    {#each ['webs', 'flanges'] as wf}
        {#if data[wf]}
            <TabPane tabId={wf} tab={wf.replace(/\b\w/, (c) => c.toUpperCase())} active={wf === 'webs'}>
                <Table hover>
                    <thead>
                        <tr>
                            <th>Material Master</th>
                            <th>Description</th>
                            <th>Grade</th>
                            <th>Program</th>
                            <th>Checked</th>
                            <th>Printed</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each data[wf] as row}
                            <tr>
                                <td>{ row.matl }</td>
                                <td>{ row.matl_desc }</td>
                                <td>{ row.grade }</td>
                                <td>{ row.program }</td>
                                <td>{ row.checked || "" }</td>
                                <td>{ row.printed || "" }</td>
                            </tr>
                        {/each}
                    </tbody>
                </Table>
            </TabPane>
        {/if}
    {/each}

    {#if data.parts}
        <TabPane tabId="parts" tab="Parts" active={!data.webs}>
            <Table hover>
                <thead>
                    <tr>
                        <th>Grade</th>
                        <th>Thickness</th>
                        <th>Sheet Name</th>
                        <th>Program</th>
                        <th>Printed</th>
                    </tr>
                </thead>
                <tbody>
                    {#each data.parts as row}
                        <tr>
                            <td>{ row.grade }</td>
                            <td>{ row.thk }</td>
                            <td>{ row.sheet }</td>
                            <td>{ row.program }</td>
                            <td>{ row.printed || "" }</td>
                        </tr>
                    {/each}
                </tbody>
            </Table>
        </TabPane>
    {/if}
</TabContent>
