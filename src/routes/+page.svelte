<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.css';
	import { invoke } from '@tauri-apps/api/tauri';
	import WatchedAssets from '$lib/components/WatchedAssets.svelte';

	$: watched_symbols = [''];

	onMount(async () => {
		watched_symbols = await invoke('get_symbols');

		console.log(watched_symbols);
	});
</script>

<WatchedAssets symbols={watched_symbols} />
