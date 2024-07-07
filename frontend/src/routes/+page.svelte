<script lang="ts">
	import PortView from '$lib/Viewers/PortView.svelte';
	import { createPortApi, getPortsApi, removePortApi, updatePortApi } from '$lib/ports';
	import type { Port } from '$lib/ports';
	import { onMount } from 'svelte';

	import Status from '$lib/Status.svelte';

	let ports: Port[] = [];
	let create_port_port = {
		name: 'fill-in-new',
		forward_type: 'Tcp',
		remote_port: 0,
		local_port: 0,
	};

	onMount(async () => {
		getPortsApi().then((data) => {
			ports = data;
		});
		console.log(ports);
	});

	async function removePort(e: CustomEvent<string>) {
		// remove the port from the list
		let success = await removePortApi(e.detail);
		if (success) {
			ports = ports.filter((port) => port.name !== e.detail);
		}
	}
	async function updatePort(e: CustomEvent<Port>) {
		// update the port in the list
		let success = await updatePortApi(e.detail);
		if (success) {
			ports = await getPortsApi();
		}
	}
	async function addPort(e: CustomEvent<Port>) {
		let success = await createPortApi(e.detail);	
		if (success) {
			ports = [...ports, e.detail];
			
		}
	}
</script>

<div class="wrapper">
	<h1>main page</h1>
	<Status />
	<PortView port={create_port_port} on:update={addPort} />
	{#each ports as port}
		<PortView {port} on:remove={removePort} on:update={updatePort} />
	{/each}
</div>
