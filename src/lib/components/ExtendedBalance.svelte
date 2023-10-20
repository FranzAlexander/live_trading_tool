<script lang="ts">
	import type { ExtendedBalances } from '$lib';

	export let data: ExtendedBalances;
</script>

<div class="border-2 border-blue-500 rounded-lg text-white">
	<h2 class="">Extended Balance</h2>
	<div class="flex-col">
		{#each Object.entries(data) as [symbol, amount] (symbol)}
			<h1 class="">{symbol}</h1>
			<p>{amount.balance}</p>
			<p>{amount.hold_trade}</p>
		{/each}
	</div>
</div>
