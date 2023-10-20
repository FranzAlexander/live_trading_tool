<script lang="ts">
	export let currenyAmount: number;

	$: orderType = 'buy';
	$: symbol = 'XRP';
	$: price = 0.493;
	$: amount = 0.0;
	$: amountPercentage = 0;
</script>

<div
	class="w-full border-2 bg-blue-500/10 border-slate-800 rounded-lg flex flex-col h-[90%] p-1 gap-5 shadow-md shadow-slate-500/30"
>
	<h2 class="text-white text-2xl font-bold">Order Panel</h2>
	<div class="flex flex-row w-full bg-blue-500/20 text-white justify-between rounded-full">
		<button
			class="w-full {orderType == 'buy' ? 'bg-blue-500' : ''}  text-lg p-2 rounded-l-full font-bold"
			on:click={() => {
				orderType = 'buy';
			}}>Buy</button
		>
		<button
			class="w-full {orderType == 'sell' ? 'bg-blue-500' : ''} text-lg p-2 rounded-r-full font-bold"
			on:click={() => {
				orderType = 'sell';
			}}>Sell</button
		>
	</div>
	<div class="flex border-blue-500/20 border-2 py-1 px-2 rounded-lg justify-between">
		<label for="limitPrice" class="text-white text-sm font-semibold">Limit Price:</label>
		<input
			name="limitPrice"
			type="number"
			class="bg-transparent outline-none text-white text-right appearance-none number-input mr-2"
			placeholder="0.0"
			bind:value={price}
		/>
		<span class="text-white"
			>{#if orderType == 'buy'}
				USD
			{:else}
				{symbol}
			{/if}</span
		>
	</div>

	<div class="flex border-blue-500/20 border-2 py-1 px-2 rounded-lg justify-between">
		<label for="tradeAmount" class="text-white text-lg font-semibold">Amount:</label>
		<input
			type="number"
			name="tradeAmount"
			class="bg-transparent outline-none text-white text-right appearance-none number-input"
			placeholder="0.0"
			bind:value={amount}
		/>
		<span class="text-white"
			>{#if orderType == 'buy'}
				USD
			{:else}
				{symbol}
			{/if}</span
		>
	</div>

	<div>
		<input type="range" name="" min="0" max="100" id="" bind:value={amountPercentage} />
	</div>
</div>

<style>
	/* Hide the spinner arrows */
	.number-input::-webkit-inner-spin-button,
	.number-input::-webkit-outer-spin-button {
		-webkit-appearance: none;
		appearance: none;
		margin: 0;
	}

	/* Optionally, you can also hide the input arrow key increment/decrement functionality */
	.number-input::-webkit-contacts-auto-fill-button {
		display: none;
	}
</style>
