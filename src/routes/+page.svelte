<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.css';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { DeltaBar, RangeBar, RangeBarPayload } from '$lib/range';

	import { listen } from '@tauri-apps/api/event';
	import { candlestickConfig, createCandlestickConfig, createMainCandleChart } from '$lib/chart';
	import type { IChartApi, ISeriesApi, Time } from 'lightweight-charts';
	import type { ExtendedBalances, ExtendedDetails, OhlcPayload } from '$lib';
	import TradingOrderPanel from '$lib/components/TradingOrderPanel.svelte';

	$: loaded = false;

	let rangeChart: IChartApi;
	let rangeBars: ISeriesApi<'Candlestick'>;

	let deltaChart: IChartApi;
	let deltaBars: ISeriesApi<'Candlestick'>;

	let oneMinChart: IChartApi;
	let oneMinBars: ISeriesApi<'Candlestick'>;
	let oneMinDeltaChart: IChartApi;
	let oneMinDeltaBars: ISeriesApi<'Candlestick'>;

	let fiveMinChart: IChartApi;
	let fiveMinBars: ISeriesApi<'Candlestick'>;
	let fiveMinDeltaChart: IChartApi;
	let fiveMinDeltaBars: ISeriesApi<'Candlestick'>;

	let rangeChartContainer: HTMLElement;

	let deltaChartContainer: HTMLElement;

	let oneMainChartContainer: HTMLElement;

	let oneMinDeltaChartContainer: HTMLElement;

	let fiveMainChartContainer: HTMLElement;

	let fiveMinDeltaChartContainer: HTMLElement;

	$: usdBalance = {} as ExtendedDetails;
	$: solBalance = {} as ExtendedDetails;

	listen('tauri://resize', (event) => {
		rangeChart.resize(rangeChartContainer.clientWidth, rangeChartContainer.clientHeight);
		deltaChart.resize(deltaChartContainer.clientWidth, deltaChartContainer.clientHeight);
		oneMinChart.resize(oneMainChartContainer.clientWidth, oneMainChartContainer.clientHeight);
		oneMinDeltaChart.resize(
			oneMinDeltaChartContainer.clientWidth,
			oneMinDeltaChartContainer.clientHeight
		);
		fiveMinChart.resize(fiveMainChartContainer.clientWidth, fiveMainChartContainer.clientHeight);
		fiveMinDeltaChart.resize(
			fiveMinDeltaChartContainer.clientWidth,
			fiveMinDeltaChartContainer.clientHeight
		);
	});

	onMount(async () => {
		const extendedBalance = (await invoke('get_extended_balance')) as ExtendedBalances;
		usdBalance = extendedBalance['ZUSD'];
		solBalance = extendedBalance['SOL'];

		const assetPairInfo = await invoke('get_tradeable_asset_pair');

		const rangeData: [RangeBar[], DeltaBar[]] = await invoke('app_start');

		rangeChart = createMainCandleChart(rangeChartContainer);
		deltaChart = createMainCandleChart(deltaChartContainer);

		oneMinChart = createMainCandleChart(oneMainChartContainer);
		oneMinDeltaChart = createMainCandleChart(oneMinDeltaChartContainer);

		fiveMinChart = createMainCandleChart(fiveMainChartContainer);
		fiveMinDeltaChart = createMainCandleChart(fiveMinDeltaChartContainer);

		rangeBars = rangeChart.addCandlestickSeries(createCandlestickConfig(2, 0.01));
		deltaBars = deltaChart.addCandlestickSeries(createCandlestickConfig(2, 0.01));
		oneMinBars = oneMinChart.addCandlestickSeries(createCandlestickConfig(2, 0.01));
		oneMinDeltaBars = oneMinDeltaChart.addCandlestickSeries(createCandlestickConfig(2, 0.01));

		fiveMinBars = fiveMinChart.addCandlestickSeries(createCandlestickConfig(2, 0.01));
		fiveMinDeltaBars = fiveMinDeltaChart.addCandlestickSeries(createCandlestickConfig(2, 0.01));

		const chartData = rangeData[0].map((item) => {
			return {
				time: item.start_time as Time, // Use the formatted time
				open: Number(item.open),
				high: Number(item.high),
				low: Number(item.low),
				close: Number(item.close)
			};
		});

		const deltaData = rangeData[1].map((item) => {
			return {
				time: item.start_time as Time,
				open: Number(item.open),
				high: Number(item.high),
				low: Number(item.low),
				close: Number(item.close)
			};
		});

		rangeBars.setData(chartData);
		deltaBars.setData(deltaData);

		rangeChart.timeScale().fitContent();
		rangeChart.timeScale().scrollToRealTime();

		deltaChart.timeScale().fitContent();
		deltaChart.timeScale().scrollToRealTime();

		oneMinChart.timeScale().fitContent();
		oneMinChart.timeScale().scrollToRealTime();

		oneMinDeltaChart.timeScale().fitContent();
		oneMinDeltaChart.timeScale().scrollToRealTime();

		fiveMinChart.timeScale().fitContent();
		fiveMinChart.timeScale().scrollToRealTime();

		fiveMinDeltaChart.timeScale().fitContent();
		fiveMinDeltaChart.timeScale().scrollToRealTime();

		loaded = true;
	});

	listen('new_bar', ({ payload }) => {
		const bar = payload as RangeBar;

		let newRangeBar = {
			time: bar.start_time as Time, // Use the formatted time
			open: Number(bar.open),
			high: Number(bar.high),
			low: Number(bar.low),
			close: Number(bar.close)
		};

		rangeBars.update(newRangeBar);
	});

	listen('new_delta_bar', ({ payload }) => {
		const deltaBar = payload as DeltaBar;

		let newDeltaBar = {
			time: deltaBar.start_time as Time,
			open: Number(deltaBar.open),
			high: Number(deltaBar.high),
			low: Number(deltaBar.low),
			close: Number(deltaBar.close)
		};

		deltaBars.update(newDeltaBar);
	});

	listen('oneMinOhlc', ({ payload }) => {
		const ohlcBar = payload as OhlcPayload;

		let newOhlcBar = {
			time: ohlcBar.time as Time,
			open: Number(ohlcBar.open),
			high: Number(ohlcBar.high),
			low: Number(ohlcBar.low),
			close: Number(ohlcBar.close)
		};

		oneMinBars.update(newOhlcBar);
	});

	listen('oneMinCVD', ({ payload }) => {
		const deltaBar = payload as DeltaBar;

		let newDeltaBar = {
			time: deltaBar.start_time as Time,
			open: Number(deltaBar.open),
			high: Number(deltaBar.high),
			low: Number(deltaBar.low),
			close: Number(deltaBar.close)
		};
		oneMinDeltaBars.update(newDeltaBar);
	});

	listen('fiveMinOhlc', ({ payload }) => {
		const ohlcBar = payload as OhlcPayload;

		let newOhlcBar = {
			time: ohlcBar.time as Time,
			open: Number(ohlcBar.open),
			high: Number(ohlcBar.high),
			low: Number(ohlcBar.low),
			close: Number(ohlcBar.close)
		};

		fiveMinBars.update(newOhlcBar);
	});

	listen('fiveMinCVD', ({ payload }) => {
		const deltaBar = payload as DeltaBar;

		let newDeltaBar = {
			time: deltaBar.start_time as Time,
			open: Number(deltaBar.open),
			high: Number(deltaBar.high),
			low: Number(deltaBar.low),
			close: Number(deltaBar.close)
		};
		fiveMinDeltaBars.update(newDeltaBar);
	});
</script>

<main class="w-[100vw] h-[100vh] overflow-none p-1 flex-col">
	<div class="w-full h-10 flex">
		<div class="flex">
			<h2 class="text-white">USD:</h2>
			<span class="text-white">${usdBalance.balance}</span>
		</div>
		<div class="flex">
			<h2 class="text-white">SOL:</h2>
			<span class="text-white">{solBalance.balance}</span>
		</div>
	</div>
	<div class="h-2/3 w-full flex">
		<div class="h-full w-1/2">
			<div class="w-full h-2/3" bind:this={rangeChartContainer} />
			<div class="w-full h-1/3" bind:this={deltaChartContainer} />
		</div>
		<div class="h-full w-1/2">
			<div class="h-1/2 w-full">
				<div class="w-full h-2/3" bind:this={oneMainChartContainer} />
				<div class="w-full h-1/3" bind:this={oneMinDeltaChartContainer} />
			</div>
			<div class="h-1/2 w-full">
				<div class="w-full h-2/3" bind:this={fiveMainChartContainer} />
				<div class="w-full h-1/3" bind:this={fiveMinDeltaChartContainer} />
			</div>
		</div>
	</div>
	<div class="w-full">
		<TradingOrderPanel />
	</div>
</main>
