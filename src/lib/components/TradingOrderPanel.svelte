<script lang="ts">
	const orderTypes: string[] = ['limit', 'market', 'stop-loss'];

	let quantity = 1;
	let price = 42.5;
	let fees = 0.12;
	let leverage = 4;
	// let orderType = 'limit'; // default to market order
	let selectedOrderType = 'limit';
	$: totalPrice = 0.0;
	$: buySell = 'buy';

	const calculateTotalPrice = () => {
		totalPrice = price * quantity * leverage + fees;
	};

	$: calculateTotalPrice;
</script>

<div class="w-1/2 text-white bg-gray-800 flex">
	<div class="px-2 mb-4">
		<label for="orderType" class="block text-sm font-medium">Order Type</label>
		<select
			id="orderType"
			bind:value={selectedOrderType}
			class="mt-1 block w-full bg-gray-700 border border-gray-600 text-white rounded-md shadow-sm"
		>
			{#each orderTypes as orderType}
				<option value={orderType}>{orderType.charAt(0).toUpperCase() + orderType.slice(1)}</option>
			{/each}
		</select>
	</div>
</div>

<!-- <div class="absolute inset-x-0 bottom-0 p-4 bg-gray-800 text-white">
	<div class="flex flex-wrap -mx-2">
		<div class="w-full md:w-1/5 px-2 mb-4">
			<label for="orderType" class="block text-sm font-medium">Order Type</label>
			<select
				id="orderType"
				bind:value={orderType}
				class="mt-1 block w-full bg-gray-700 border border-gray-600 text-white rounded-md shadow-sm"
			>
				<option value="market">Market</option>
				<option value="limit">Limit</option>
				<option value="stop">Stop</option>
			</select>
		</div>

		<div class="w-full md:w-1/5 px-2 mb-4">
			<label for="quantity" class="block text-sm font-medium text-gray-700">Quantity</label>
			<input
				type="number"
				id="quantity"
				min="1"
				bind:value={quantity}
				class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
			/>
		</div>

		{#if orderType === 'limit'}
			<div class="w-full md:w-1/5 px-2 mb-4">
				<label for="price" class="block text-sm font-medium text-gray-700">Limit Price</label>
				<input
					type="number"
					id="price"
					bind:value={price}
					step="0.01"
					class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
				/>
			</div>
		{/if}

		<div class="w-full md:w-1/5 px-2 mb-4">
			<label for="leverage" class="block text-sm font-medium">Leverage</label>
			<input
				type="number"
				id="leverage"
				min="1"
				bind:value={leverage}
				class="mt-1 block w-full pl-3 pr-10 py-2 text-base bg-gray-700 border border-gray-600 text-white rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
			/>
		</div>

		<div class="w-full md:w-1/5 px-2 mb-4">
			<label for="fees" class="block text-sm font-medium">Fees</label>
			<input
				type="number"
				id="fees"
				min="0"
				bind:value={fees}
				step="0.01"
				class="mt-1 block w-full pl-3 pr-10 py-2 text-base bg-gray-700 border border-gray-600 text-white rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
			/>
		</div>
	</div>

	<div class="text-lg font-bold p-2">
		Total Price: <span class="text-green-400">${totalPrice.toFixed(2)}</span>
	</div>

	<button
		class="mt-4 w-full md:w-auto bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded"
	>
		Place Order
	</button>
</div> -->
