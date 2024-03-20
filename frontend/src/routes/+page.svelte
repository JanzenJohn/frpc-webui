<script lang="ts">
	import PortView from '$lib/Viewers/PortView.svelte';
	import { getPortsApi, removePortApi } from '$lib/ports';
	import type { Port } from '$lib/ports';
	import { onMount } from 'svelte';
	import CreatePort from '$lib/CreatePort.svelte';
	import Status from '$lib/Status.svelte';

	let ports: Port[] = [];

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
	async function addPort(e: CustomEvent<Port>) {
		ports = [...ports, e.detail];
	}
</script>

<div class="wrapper">
	<h1>main page</h1>
	<Status />
	<CreatePort on:add={addPort} />
	{#each ports as port}
		<PortView {port} on:remove={removePort} />
	{/each}
</div>
