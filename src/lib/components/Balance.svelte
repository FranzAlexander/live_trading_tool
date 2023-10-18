<!-- <script lang="ts">
	import {
		isExtendedBalances,
		type ExtendedBalance,
		type ExtendedBalances,
		type TradeBalance
	} from '$lib';

	export let name: string;
	export let data: ExtendedBalances | TradeBalance;

	let ext = isExtendedBalances(data);
	let extendedData: ExtendedBalances = ext ? (data as ExtendedBalances) : {};
	let tradeData: TradeBalance = ext ? ({} as TradeBalance) : (data as TradeBalance);
</script>

<div class="border-2 border-blue-500 rounded-lg">
	<h2 class="text-white">{name}</h2>
	<div class="flex-col">
		{#if ext}
			{#each Object.keys(extendedData) as currency}
				<div>
					<b>{currency}</b>: {extendedData[currency].balance} / {extendedData[currency].hold_trade}
				</div>
			{/each}
		{:else}
			{#each Object.keys(tradeData) as key}
				{#if tradeData[key] !== undefined}
					<div>
						<b>{key}</b>: {tradeData[key]}
					</div>
				{/if}
			{/each}
		{/if}
	</div>
</div> -->
