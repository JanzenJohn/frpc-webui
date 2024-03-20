<script lang="ts">
	import type { Port } from '$lib/ports';
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	export let port: Port;

	function removePort(name: string) {
		dispatch('remove', name);
	}
</script>

<div class="wrapper">
	<div class="top g">
		<span class="name">{port.name}</span>
		<span>{port.forward_type}</span>
		<button on:click={() => removePort(port.name)}>X</button>
	</div>
	<div class="divider"></div>
	<div class="bottom g">
		<span>remote: {port.remote_port}</span>
		<span> ➤ </span>
		<span>local: {port.local_port}</span>
	</div>
</div>

<style>
	.wrapper {
		width: 200px;
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
</style>
