<script lang="ts">
	import { createPortApi, ForwardType, type Port } from '$lib/ports';
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	async function createPort(event: Event) {
		event.preventDefault();
		const form = event.target as HTMLFormElement;
		const formData = new FormData(form);
		const data: Port = {
			name: formData.get('name') as string,
			forward_type: formData.get('protocol') as ForwardType,
			local_port: parseInt(formData.get('source_port') as string),
			remote_port: parseInt(formData.get('target_port') as string)
		};
		if (await createPortApi(data)) {
			dispatch('add', data);
			form.reset();
		} else {
			console.error('Failed to create port');
		}
	}
</script>

<div class="wrapper">
	<form on:submit|preventDefault={createPort}>
		<input type="text" name="name" placeholder="Name" required />
		<input type="number" name="source_port" placeholder="Source" min="1" max="65536" required />
		<input type="number" name="target_port" placeholder="Target" min="1" max="65536" required />
		<select name="protocol" required>
			<option value="Tcp">TCP</option>
			<option value="Udp">UDP</option>
		</select>
		<button type="submit">Create</button>
	</form>
</div>
