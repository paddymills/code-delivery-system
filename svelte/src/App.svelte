<script lang="ts">
	import { Alert, Button, Icon, Spinner, Styles } from 'sveltestrap';
	import JobShipSelector from './components/JobShipSelector.svelte';
	import TabbedView from './components/TabbedView.svelte';

	import { invoke } from '@tauri-apps/api/tauri';

    let jobs = null;
	let data = null;

	async function init() {
		await invoke('get_projects')
			.then((result: { msg: string, payload: [any] }) => {
				console.log(result.msg);
				jobs = result.payload;
			})
			.catch((error) => console.error(error));
	}

	async function handleClick(event) {
		const args = {
			project: {
				name: event.detail.job,
				shipments: [event.detail.shipment]
			}
		};

		await invoke('load_job_shipment', args)
			.then((result: { msg: string, payload: any }) => {
				console.log(result.msg);
				data = result.payload;
			})
			.catch((error) => console.error(error));
	};
</script>

<main class="d-flex flex-column align-items-center p-5 gap-3">
	{#await init()}
		<Spinner color="primary"/>
	{:then}
		<JobShipSelector jobs={jobs} on:load={handleClick} />
	{:catch error}
		<Alert color="danger">Failure retreiving jobs: {error}</Alert>
	{/await}
	
	<div class="p-3 position-relative border rounded">
		{#if data}
			<div class="position-absolute top-0 end-0 p-1">
				<Button class="btn-close" style="background: transparent url('icons/filter_icon.svg');"></Button>
				<Button class="btn-close" on:click={() => data=null} />
			</div>

			{#key data}
				<!--
					force TabbedView to be destroyed and recreated on data change
					needed to render the first tab's active state
				-->
				<TabbedView data={data} />
			{/key}
		{:else}
			<p class="m-0">no data</p>
		{/if}
	</div>
</main>

<Styles />