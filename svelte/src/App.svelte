<script>
	import { Alert, Spinner, Styles } from 'sveltestrap';
	import JobShipSelector from './components/JobShipSelector.svelte';
	import TabbedView from './components/TabbedView.svelte';

	import { invoke } from '@tauri-apps/api/tauri';

    let jobs = null;
	let data = null;

	async function init() {
		await invoke('get_projects')
			.then((result) => {
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
			.then((result) => {
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
	
	<div class="mx-5 p-3 d-flex justify-content-center border rounded">
		{#if data}
			<TabbedView data={data} />
		{:else}
			<p class="m-0">no data</p>
		{/if}
	</div>
</main>

<Styles />