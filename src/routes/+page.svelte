<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.css';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { DeltaBar, RangeBar, RangeBarPayload } from '$lib/range';

	import { listen } from '@tauri-apps/api/event';
	import { candlestickConfig, createCandlestickConfig, createMainCandleChart } from '$lib/chart';
	import type { IChartApi, ISeriesApi, Time } from 'lightweight-charts';
	import type { OhlcPayload } from '$lib';

	$: loaded = false;

	let rangeChart: IChartApi;
	let rangeBars: ISeriesApi<'Candlestick'>;

	let deltaChart: IChartApi;
	let deltaBars: ISeriesApi<'Candlestick'>;

	let oneMinChart: IChartApi;
	let oneMinBars: ISeriesApi<'Candlestick'>;

	let rangeChartContainer: HTMLElement;

	let deltaChartContainer: HTMLElement;

	let oneMainChartContainer: HTMLElement;

	listen('tauri://resize', (event) => {
		rangeChart.resize(rangeChartContainer.clientWidth, rangeChartContainer.clientHeight);
		deltaChart.resize(deltaChartContainer.clientWidth, deltaChartContainer.clientHeight);
		oneMinChart.resize(oneMainChartContainer.clientWidth, oneMainChartContainer.clientHeight);
	});

	onMount(async () => {
		const rangeData: [RangeBar[], DeltaBar[]] = await invoke('app_start');

		rangeChart = createMainCandleChart(rangeChartContainer);
		deltaChart = createMainCandleChart(deltaChartContainer);
		oneMinChart = createMainCandleChart(oneMainChartContainer);

		rangeBars = rangeChart.addCandlestickSeries(createCandlestickConfig(2, 0.01));
		deltaBars = deltaChart.addCandlestickSeries(createCandlestickConfig(2, 0.01));
		oneMinBars = oneMinChart.addCandlestickSeries(createCandlestickConfig(2, 0.01));

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

		console.log(ohlcBar);

		let newOhlcBar = {
			time: ohlcBar.time as Time,
			open: Number(ohlcBar.open),
			high: Number(ohlcBar.high),
			low: Number(ohlcBar.low),
			close: Number(ohlcBar.close)
		};

		oneMinBars.update(newOhlcBar);
	});
</script>

<main class="w-[100vw] h-[100vh] overflow-none p-1 flex-col">
	<div class="h-2/3 w-full flex">
		<div class="h-full w-1/2">
			<div class="w-full h-2/3" bind:this={rangeChartContainer} />
			<div class="w-full h-1/3" bind:this={deltaChartContainer} />
		</div>
		<div class="h-full w-1/2">
			<div class="w-full h-2/3" bind:this={oneMainChartContainer} />
		</div>
	</div>
</main>

<style>
	/* main {
		width: 100vw;
		height: 100vh;
		/* display: flex; */
	/* flex-direction: column; */
	/* gap: 1rem;
		grid-template-columns: 15% minmax(0, 2fr) minmax(0, 1fr) 15%;
		grid-template-rows: 10% 40% repeat(2, minmax(0, 1fr)); 
		padding: 1rem;
		overflow: hidden; }*/

	/* main > div:nth-child(1) {
		height: 50%;
	}

	main > div:nth-child(2) {
		height: 30%;
	} */
</style>
