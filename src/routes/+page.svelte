<script lang="ts">
	import '../app.css';

	import type { ExtendedBalances, OhlcEventPayload, TradeBalance } from '$lib';
	import Balance from '$lib/components/Balance.svelte';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/tauri';
	import {
		createChart,
		type ChartOptions,
		type DeepPartial,
		type IChartApi,
		ColorType,
		CrosshairMode,
		LineStyle,
		type ISeriesApi,
		type Time
	} from 'lightweight-charts';
	import { onMount } from 'svelte';
	import ExtendedBalance from '$lib/components/ExtendedBalance.svelte';
	import { setEmaHistory } from '$lib/ema';
	import CoinButton from '$lib/components/CoinButton.svelte';
	import { setMacdHistory, type MacdHistory } from '$lib/indicators';
	import MacdChart from '$lib/components/MacdChart.svelte';
	import OrderPanel from '$lib/components/OrderPanel.svelte';
	// 5C2EDD nEXT COLOUR

	let coins: ('BTC' | 'ETH' | 'XRP')[] = ['BTC', 'ETH', 'XRP'];

	let chart: IChartApi;
	let candlestickSeries: ISeriesApi<'Candlestick'>;
	let volumeSeries: ISeriesApi<'Histogram'>;
	let vwapSeries: ISeriesApi<'Line'>;
	let emaSeries: ISeriesApi<'Line'>;

	let extendedBalances: ExtendedBalances = {};
	let trade_balances: TradeBalance;

	let container: HTMLElement;
	let macdHistory: MacdHistory[];

	let currenyAmount = 0.0;

	listen('tauri://resize', (event) => {
		chart.resize(container.clientWidth, container.clientHeight);
	});

	// Listen to the `message-stream` event from the backend
	listen('message-stream', (event) => {
		console.log(event.payload);
		// Handle the streamed message as required
	});

	invoke('init_process');

	$: loaded = false;

	onMount(async () => {
		chart = createChart(container, {
			layout: {
				background: {
					type: ColorType.Solid,
					color: '#000'
				},
				textColor: '#DDD'
			},
			grid: { vertLines: { color: '#444' }, horzLines: { color: '#444' } },
			crosshair: {
				mode: CrosshairMode.Normal,
				vertLine: {
					color: '#C3BCDB44',
					style: LineStyle.Solid,
					width: 4,
					labelBackgroundColor: '#0000CC'
				},
				horzLine: {
					color: '#0000CC',
					labelBackgroundColor: '#0000CC'
				}
			}
		});

		volumeSeries = chart.addHistogramSeries({
			color: '#26a69a',
			baseLineWidth: 2,
			priceFormat: {
				type: 'volume',
				minMove: 0.0002
			},
			priceScaleId: ''
		});

		vwapSeries = chart.addLineSeries({
			color: '#6699CC',
			lineWidth: 2,
			priceFormat: { type: 'volume', precision: 5, minMove: 0.0002 }
		});

		candlestickSeries = chart.addCandlestickSeries({
			upColor: '#00CC00',
			downColor: '#CC0000',
			borderVisible: false,
			wickUpColor: '#00CC00',
			wickDownColor: '#CC0000',

			priceFormat: { type: 'price', precision: 5, minMove: 0.0002 }
		});

		emaSeries = chart.addLineSeries({
			color: '#8a00e6',
			lineWidth: 2,

			priceFormat: { type: 'price', precision: 5, minMove: 0.0002 }
		});

		chart.priceScale('').applyOptions({
			scaleMargins: {
				top: 0.8,
				bottom: 0
			}
		});

		await invoke('get_ohlc_history').then(async (result) => {
			let ohlcData = result as OhlcEventPayload[];
			ohlcData.sort((a, b) => a.time - b.time);

			const chartData = ohlcData.map((item) => {
				return {
					time: item.time as Time,
					open: Number(item.open),
					high: Number(item.high),
					low: Number(item.low),
					close: Number(item.close)
				};
			});

			const volumeData = ohlcData.map((item) => {
				return {
					time: item.time as Time,
					value: Number(item.volume),
					color: Number(item.open) > Number(item.close) ? '#CC0000' : '#00CC00'
				};
			});

			const vwapData = ohlcData.map((item) => {
				return {
					time: item.time as Time,
					value: Number(item.vwap)
				};
			});

			candlestickSeries.setData(chartData);
			volumeSeries.setData(volumeData);

			const emaHistory = await setEmaHistory(ohlcData);

			const emaData = emaHistory.map((item) => {
				return {
					time: item.time as Time,
					value: Number(item.value)
				};
			});

			macdHistory = await setMacdHistory(ohlcData);

			// console.log(emaData);

			emaSeries.setData(emaData);

			// vwapSeries.setData(vwapData);
		});

		const extendedResult: ExtendedBalances = await invoke('get_extended_balance');
		console.log(extendedResult);

		for (const [key, value] of Object.entries(extendedResult)) {
			const newKey = key.startsWith('Z') || key.startsWith('X') ? key.substring(1) : key;
			extendedBalances[newKey] = value;
		}

		const symbols = ['XRPUSD'];
		await invoke('get_tradeable_assets', { symbols: symbols });

		currenyAmount = extendedBalances['USD'].balance;

		trade_balances = await invoke('get_trade_balance');

		candlestickSeries.applyOptions({
			wickUpColor: '#00CC00',
			upColor: '#00CC00',
			wickDownColor: '#CC0000',
			downColor: '#CC0000',
			borderVisible: false,
			priceFormat: { type: 'price', precision: 5 }
		});
		chart.timeScale().fitContent();

		loaded = true;
	});
</script>

<main>
	<div class="w-full flex gap-10">
		{#each coins as coin}
			<CoinButton coinName={coin} />
		{/each}
	</div>

	<div class="flex flex-row w-full" id="main-chart-container">
		<div class="w-4/5" bind:this={container} />
		<div class="w-1/6">
			<OrderPanel bind:currenyAmount />
		</div>
	</div>
	<div class="flex justify-between">
		{#if loaded}
			<MacdChart initialOhlc={macdHistory} />
		{/if}

		<ExtendedBalance data={extendedBalances} />
	</div>
</main>

<style>
	main {
		width: 100vw;
		height: 100vh;
		display: flex;
		flex-direction: column;
		box-sizing: border-box;
		padding: 1rem;
		overflow: hidden; /* Add this to prevent overflow if contents are too big for their containers */
	}

	/* .chart-container {
		grid-row: 2 / 4; 
		grid-column: 2 / 4; 
	} */

	/* #ext-balance {
		grid-row: 2;
		border: 1px;
	} */

	main > div:nth-child(1) {
		height: 5%;
	}

	main > div:nth-child(2) {
		height: 50%;
	}

	main > div:nth-child(3) {
		height: 40%;
	}
</style>
