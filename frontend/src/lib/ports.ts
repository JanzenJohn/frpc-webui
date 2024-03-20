export interface Port {
	local_port: number;
	remote_port: number;
	name: string;
	forward_type: ForwardType;
}

export enum ForwardType {
	Tcp = 'Tcp',
	Udp = 'Udp'
}

interface PortBase {
	[key: string]: Port;
}

export async function getPortsApi(): Promise<Port[]> {
	// convert hashmap to array
	let port_base: PortBase = await fetch('/api/ports').then((r) => r.json());
	return Object.entries(port_base).map(([key, value]) => {
		return {
			local_port: value.local_port,
			remote_port: value.remote_port,
			name: key,
			forward_type: value.forward_type
		};
	});
}

export function removePortApi(name: string): Promise<boolean> {
	return new Promise<boolean>(async (resolve, reject) => {
		fetch(`/api/ports/${name}`, { method: 'DELETE' })
			.then((r) => {
				if (r.status === 200) {
					resolve(true);
				} else {
					resolve(false);
				}
			})
			.catch((e) => {
				resolve(false);
			});
	});
}

export function createPortApi(port: Port): Promise<boolean> {
	return new Promise<boolean>(async (resolve, reject) => {
		fetch(`/api/ports/${port.name}`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(port)
		})
			.then((r) => {
				if (r.status === 201) {
					resolve(true);
				} else {
					resolve(false);
				}
			})
			.catch((e) => {
				resolve(false);
			});
	});
}
