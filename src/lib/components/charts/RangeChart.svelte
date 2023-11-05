<script lang="ts">
	import { candlestickConfig, createMainCandleChart } from '$lib/chart';
	import type { RangeBar } from '$lib/range';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import type { IChartApi, ISeriesApi, Time } from 'lightweight-charts';
	import { onMount } from 'svelte';

	export let rangeBarData: { time: Time; open: number; high: number; low: number; close: number }[];

	let chart: IChartApi;
	let rangeBars: ISeriesApi<'Candlestick'>;

	let chartContainer: HTMLElement;

	let unlisten: Promise<UnlistenFn>;

	onMount(async () => {
		chart = createMainCandleChart(chartContainer);
		rangeBars = chart.addCandlestickSeries(candlestickConfig);

		rangeBars.setData(rangeBarData);

		chart.timeScale().fitContent();
		chart.timeScale().scrollToRealTime();

		unlisten = listen('new_bar', ({ payload }) => {
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
	});
</script>

<div class="flex-col">
	<div bind:this={chartContainer} />
</div>
