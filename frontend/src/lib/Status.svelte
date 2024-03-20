<script lang="ts">
	import { get } from './api';
	import { onMount } from 'svelte';

	let status = false;
	async function getStatus() {
		while (true) {
			status = await get('/api/status');
			await new Promise((r) => setTimeout(r, 1000));
		}
	}
	async function startProcess() {
		await get('/api/restart');
	}
	async function stopProcess() {
		await get('/api/stop');
	}
	onMount(getStatus);
</script>

<div>
	<h1>Status</h1>
	{#if status}
		<p>Running</p>
	{:else}
		<p>Not Running</p>
	{/if}
	<button on:click={startProcess}>
		{#if status}
			Restart
		{:else}
			Start
		{/if}
	</button>
	<button on:click={stopProcess}>Stop</button>
</div>
