
<script lang="ts">
    import { TabContent, TabPane } from 'sveltestrap';
    import CdsTable from './CdsTable.svelte';

    export let data;

    const tabsWithData = () => ['webs', 'flanges', 'parts'].filter(t => data[t]);

    function getHeaderAndRows(tabName: string) {
        switch (tabName) {
            case 'webs':
            case 'flanges':
                return {
                    header: [ "Material Master", "Description", "Grade", "Program", "Checked", "Printed" ],
                    rows: data[tabName].map(row => [
                        row.sheet ? row.sheet.matl : "",
                        row.sheet ? row.sheet.matl_desc : "",
                        row.sheet ? row.sheet.grade : "",
                        row.program || "",
                        row.checked || "",
                        row.printed || "",
                    ])
                }
        
            case 'parts':
                return {
                    header: [ "Grade", "Thickness", "Sheet", "Program", "Printed" ],
                    rows: data[tabName].map(row => [
                        row.sheet ? row.sheet.grade : "",
                        row.sheet ? row.sheet.thk : "",
                        row.sheet ? row.sheet.ame : "",
                        row.program || "",
                        row.printed || "",
                    ])
                }

            default:
                console.log("Unmatched tab name: " + tabName);
        }
    }
</script>

<TabContent>
    {#each tabsWithData() as tab, id}
        {@const properName = tab.replace(/\b\w/, (c) => c.toUpperCase())}
        <TabPane tabId={tab} tab={properName} active={id === 0}>
            <CdsTable tableData={getHeaderAndRows(tab)} />
        </TabPane>
    {/each}
</TabContent>
