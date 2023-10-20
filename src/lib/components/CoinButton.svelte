<script lang="ts">
	import BTC from '$lib/icons/BTC.svelte';
	import ETH from '$lib/icons/ETH.svelte';
	import XRP from '$lib/icons/XRP.svelte';

	type CoinName = 'BTC' | 'ETH' | 'XRP';

	export let coinName: CoinName;

	const components: Record<CoinName, any> = {
		BTC,
		ETH,
		XRP
	};
</script>

<button class="text-white flex items-center gap-2 border-2 p-2 rounded-lg border-sky-500">
	<svelte:component this={components[coinName]} />
	<h1 class="text-lg">{coinName}</h1>
</button>
