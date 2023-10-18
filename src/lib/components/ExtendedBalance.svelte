<script lang="ts">
	import type { ExtendedBalances } from '$lib';

	export let data: ExtendedBalances;
	console.log(data);
</script>

<div class="border-2 border-blue-500 rounded-lg">
	<h2 class="text-white">Extended Balance</h2>
	<div class="flex-col">
		{#each Object.entries(data) as [symbol, amount] (symbol)}
			<h1>{symbol}</h1>
			<p>{amount}</p>
		{/each}
	</div>
</div>
