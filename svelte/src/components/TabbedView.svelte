
<!--
    TODO: add filters:
        - machine
        - girder group (webs/flanges)
    TODO: code mover (different interface for production control?)
    TODO: parts on nest tooltips
    TODO: program link
-->

<script lang="ts">
    import { TabContent, TabPane } from 'sveltestrap';
    import CdsTable from './CdsTable.svelte';

    export let data;

    const main_header = [ "Material Master", "Description", "Grade", "Program", "Checked", "Printed" ];
    const parts_header = [ "Grade", "Thickness", "Sheet", "Program", "Printed" ];

    const tabsWithData = () => {
        return ['webs', 'flanges', 'parts'].filter(t => data[t]);
    }

    function getHeaderAndRows(tabName: string) {
        switch (tabName) {
            case 'webs':
            case 'flanges':
                return {
                    header: main_header,
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
                    header: parts_header,
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
    {#each tabsWithData() as tab}
        <TabPane tabId={tab} tab={tab.replace(/\b\w/, (c) => c.toUpperCase())} active={tab === tabsWithData()[0]}>
            <CdsTable tableData={getHeaderAndRows(tab)} />
        </TabPane>
    {/each}
</TabContent>
