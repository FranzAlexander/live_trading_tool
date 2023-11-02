<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.css';
	import { invoke } from '@tauri-apps/api/tauri';
	import WatchedAssets from '$lib/components/WatchedAssets.svelte';
	import type { DeltaBar, RangeBar } from '$lib/range';

    $: loaded = false;

	onMount(async () => {
		const rangeData: [RangeBar[], DeltaBar[]] = await invoke('app_start');
		console.log(rangeData);
	});
</script>

<!-- <WatchedAssets symbols={watched_symbols} /> -->
