<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.css';
	import { invoke } from '@tauri-apps/api/tauri';
	import WatchedAssets from '$lib/components/WatchedAssets.svelte';
	import type { DeltaBar, RangeBar, RangeBarPayload } from '$lib/range';

	import { listen } from '@tauri-apps/api/event';
	import { candlestickConfig, createMainCandleChart } from '$lib/chart';
	import type { IChartApi, ISeriesApi, Time } from 'lightweight-charts';

	$: loaded = false;

	let rangeChart: IChartApi;
	let rangeBars: ISeriesApi<'Candlestick'>;

	let deltaChart: IChartApi;
	let deltaBars: ISeriesApi<'Candlestick'>;

	let rangeChartContainer: HTMLElement;

	let deltaChartContainer: HTMLElement;

	let rangeBarCandles: { time: Time; open: number; high: number; low: number; close: number }[];

	listen('tauri://resize', (event) => {
		rangeChart.resize(rangeChartContainer.clientWidth, rangeChartContainer.clientHeight);
	});

	onMount(async () => {
		const rangeData: [RangeBar[], DeltaBar[]] = await invoke('app_start');

		rangeChart = createMainCandleChart(rangeChartContainer);
		deltaChart = createMainCandleChart(deltaChartContainer);

		rangeBars = rangeChart.addCandlestickSeries(candlestickConfig);
		deltaBars = deltaChart.addCandlestickSeries(candlestickConfig);

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

		loaded = true;
	});

	listen('new_bar', ({ payload }) => {
		const bar = payload as RangeBar;
		console.log(bar);

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
		console.log(deltaBar);

		let newDeltaBar = {
			time: deltaBar.start_time as Time,
			open: Number(deltaBar.open),
			high: Number(deltaBar.high),
			low: Number(deltaBar.low),
			close: Number(deltaBar.close)
		};

		deltaBars.update(newDeltaBar);
	});
</script>

<main>
	<div bind:this={rangeChartContainer} />
	<div bind:this={deltaChartContainer} />
</main>

<style>
	main {
		width: 100vw;
		height: 100vh;
		/* display: flex; */
		/* flex-direction: column; */
		/* gap: 1rem;
		grid-template-columns: 15% minmax(0, 2fr) minmax(0, 1fr) 15%;
		grid-template-rows: 10% 40% repeat(2, minmax(0, 1fr)); */
		padding: 1rem;
		overflow: hidden; /* Add this to prevent overflow if contents are too big for their containers */
	}

	main > div:nth-child(1) {
		height: 50%;
	}

	main > div:nth-child(2) {
		height: 30%;
	}
</style>
