export async function get<T>(url: string): Promise<T> {
	return await (await fetch(url)).json();
}
