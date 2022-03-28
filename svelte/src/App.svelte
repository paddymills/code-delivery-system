<script>
	import {
		Button,
		Card,
		CardBody,
		Container,
		ButtonDropdown,
		DropdownItem,
		DropdownMenu,
		DropdownToggle,
		Styles
	} from 'sveltestrap';

	import { invoke } from '@tauri-apps/api/tauri';

	const jobs = [
		"1200055",
		"1200248",
		"1210105",
		"1210117",
	];
	const shipments = [ 1, 2, 3, 4, ];

	const load_enabled = false;

	const handleClick = async () => {
		await invoke('load_job_shipment')
			.then((message) => console.log(message))
			.catch((error) => console.error(error));
	};
</script>

<main class="d-flex flex-column p-5 gap-3">
	<Container class="d-flex justify-content-center gap-3">
		<ButtonDropdown>
			<DropdownToggle color="primary" caret>Job</DropdownToggle>
			<DropdownMenu>
				{#each jobs as job}
					<DropdownItem>{job}</DropdownItem>
				{/each}
			</DropdownMenu>
		</ButtonDropdown>
		<ButtonDropdown>
			<DropdownToggle color="primary" caret>Shipment</DropdownToggle>
			<DropdownMenu>
				{#each shipments as shipment}
					<DropdownItem>{shipment}</DropdownItem>
				{/each}
			</DropdownMenu>
		</ButtonDropdown>
		<Button color="primary" disabled={!load_enabled} on:click={handleClick}>Load</Button>
	</Container>
	<Card class="mx-5">
		<CardBody>
			<p>test</p>
		</CardBody>
	</Card>
</main>

<Styles />