<!-- <script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	$: isOpen = false;

	$: assets = [];

	async function getAssets() {
		invoke('get_asset_info').then((res) => {
			isOpen = true;
			assets = res;
		});
	}
</script>

<div class="relative">
	<button
		on:click={getAssets}
		class="text-white border-2 border-blue-500 p-1 text-lg rounded-xl bg-blue-500/20">Assets</button
	>

	{#if isOpen}
		<div class="absolute mt-2 w-48 rounded-md shadow-lg bg-blue-500/20 max-h-48 overflow-y-auto">
			{#each assets as asset}
				<p>{asset}</p>
			{/each}
		</div>
	{/if}
</div> -->
