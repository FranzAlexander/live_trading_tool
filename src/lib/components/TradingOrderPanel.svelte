<script lang="ts">
	export let usdAmount: number;
	export let solAmount: number;

	export let tickPrice: number;

	let percentageOfFunds: number = 1;

	enum OrderType {
		Market = 'market',
		Limit = 'limit',
		StopLoss = 'stop-loss',
		TakeProfit = 'take-profit',
		StopLossLimit = 'stop-loss-limit',
		TakeProfitLimit = 'take-profit-limit',
		SettlePosition = 'settle-position'
	}

	let orderType: OrderType = OrderType.Limit;
	let side: 'buy' | 'sell' = 'buy';
	let volume: number = 0.0;
	let leverage: number = 1;
	let quantity = 1.0;

	$: volume = ((usdAmount * percentageOfFunds) / 100) * leverage;

	let adjustedPrice: number = tickPrice;

	$: adjustedPrice = side === 'buy' ? tickPrice - 0.01 : tickPrice + 0.01;
</script>

<div class="w-1/2 text-white bg-gray-800 flex-col">
	<div class="w-full p-2 flex justify-evenly">
		<div>
			<label for="side">Side:</label>
			<select id="side" bind:value={side}>
				<option value="buy">Buy</option>
				<option value="sell">Sell</option>
			</select>
		</div>
		<div>
			<label for="orderType" class="block text-gray-700 text-sm font-bold mb-2">Order Type:</label>
			<select
				id="orderType"
				bind:value={orderType}
				class="shadow border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
			>
				{#each Object.values(OrderType) as type}
					<option value={type}>{type}</option>
				{/each}
			</select>
		</div>
		<div class="mb-4">
			<label for="leverage" class="block text-gray-300 text-sm font-bold mb-2">Leverage:</label>
			<input
				type="number"
				id="leverage"
				bind:value={leverage}
				min="1"
				max="4"
				class="shadow appearance-none border rounded w-full py-2 px-3 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 text-white"
			/>
		</div>
	</div>

	<div class="mb-6">
		<label for="adjustedPrice" class="block text-gray-300 text-sm font-bold mb-2">Price:</label>
		<input
			type="number"
			id="adjustedPrice"
			bind:value={adjustedPrice}
			step="any"
			class="shadow appearance-none border rounded w-full py-2 px-3 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 text-white"
			readonly
		/>
	</div>
	<div class="mb-4">
		<label for="fundsPercentage" class="block text-gray-300 text-sm font-bold mb-2"
			>Use Funds (%): {percentageOfFunds}%</label
		>
		<input
			type="range"
			id="fundsPercentage"
			min="0"
			max="100"
			bind:value={percentageOfFunds}
			class="w-full accent-blue-500"
		/>
	</div>

	<div class="mb-6">
		<label for="volume" class="block text-gray-300 text-sm font-bold mb-2"
			>Volume (e.g., SOL):</label
		>
		<input
			type="number"
			id="volume"
			bind:value={volume}
			min="0"
			step="any"
			readonly
			class="shadow appearance-none border rounded w-full py-2 px-3 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 text-white"
		/>
	</div>

	<button
		type="submit"
		class="w-full bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
	>
		Place Order
	</button>
</div>
