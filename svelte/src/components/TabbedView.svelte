<script>
    import { TabContent, TabPane, Table } from 'sveltestrap';

    export let data;
</script>

<TabContent>
    {#if data.webs}
        <TabPane tabId="webs" tab="Webs">
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
                    {#each data.webs as row}
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

    {#if data.flgs}
        <TabPane tabId="flgs" tab="Flanges">
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
                    {#each data.flgs as row}
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

    {#if data.parts}
        <TabPane tabId="parts" tab="Parts">
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
