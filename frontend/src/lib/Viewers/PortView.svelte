

<script lang="ts">
	import type { Port } from '$lib/ports';
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	export let port: Port;

	function removePort(name: string) {
		dispatch('remove', name);
	}
	function updatePort(port: Port) {
		dispatch('update', port);
	}
</script>

<div class="wrapper">
	<div class="top g">
		<div class="left-content">
			<input class="name" bind:value={port.name}>
		</div>

		<div class="right-content">
			<select bind:value={port.forward_type}>
				<option value="Tcp">TCP</option>
				<option value="Udp">UDP</option>
			</select>
			<button on:click={() => removePort(port.name)}>X</button>
			<button on:click={() => updatePort(port)}>💾</button>
		</div>
	</div>
	<div class="divider"></div>
	<div class="bottom g">
		<span>VPS: <input type="number" bind:value={port.remote_port}></span>
		<span> ➤ </span>
		<span>LOCAL: <input type="number" bind:value={port.local_port}></span>
	</div>
</div>

<style>
	.wrapper {
		width: fit-content;
		padding: 20px;
		margin: 10px;
		background-color: antiquewhite;
		border-radius: 15px;
	}
	.divider {
		height: 1px;
		background-color: #000;
		margin-top: 5px;
		margin-bottom: 10px;
	}
	.g {
		display: flex;
		justify-content: space-between;
		text-align: center;
	}
	input[type="number"]  {
		width: 60px;
	}
</style>
