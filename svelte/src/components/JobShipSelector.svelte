<script>
    import {
		Button,
		ButtonDropdown,
		Container,
		DropdownItem,
		DropdownMenu,
		DropdownToggle,
	} from 'sveltestrap';

    import { createEventDispatcher } from 'svelte';
    export let jobs;
    let job = { name: "Job", shipments: [] };
    let shipment = null;

    const dispatch = createEventDispatcher();

    function handleJobSelect(id) {
        job = jobs[id];
        shipment = null;
    }

	function handleClick() {
        dispatch('load', {
            job: job.name,
            shipment: shipment
        });
	};
</script>

<Container class="d-flex justify-content-center gap-3">
    <!-- job dropdown menu -->
    <ButtonDropdown>
        <DropdownToggle color="primary" caret>{job.name || 'Job'}</DropdownToggle>
        <DropdownMenu>
            {#each jobs as { name }, id}
                <DropdownItem on:click={() => handleJobSelect(id)}>{name}</DropdownItem>
            {/each}
        </DropdownMenu>
    </ButtonDropdown>

    <!-- shipment dropdown menu -->
    <ButtonDropdown>
        <DropdownToggle color="primary" caret>{shipment || 'Shipment'}</DropdownToggle>
        <DropdownMenu>
            {#each job.shipments as ship}
                <DropdownItem on:click={() => shipment=ship}>{ship}</DropdownItem>
            {:else}
                <DropdownItem disabled>Select a job first</DropdownItem>
            {/each}
        </DropdownMenu>
    </ButtonDropdown>

    <!-- load button -->
    <Button color="primary" disabled={!shipment} on:click={handleClick}>Load</Button>
</Container>
