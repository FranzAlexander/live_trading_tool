<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.css';
	import { invoke } from '@tauri-apps/api/tauri';
	import WatchedAssets from '$lib/components/WatchedAssets.svelte';
	import type { DeltaBar, RangeBar } from '$lib/range';
	import {
		ColorType,
		createChart,
		CrosshairMode,
		LineStyle,
		type IChartApi,
		type ISeriesApi,
		type Time
	} from 'lightweight-charts';

	$: loaded = false;

	let chart: IChartApi;
	let rangeBars: ISeriesApi<'Candlestick'>;

	let chartContainer: HTMLElement;

	onMount(async () => {
		const rangeData: [RangeBar[], DeltaBar[]] = await invoke('app_start');

		console.log(rangeData[0]);
		

		chart = createChart(chartContainer, {
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
			},
			timeScale: {
				timeVisible: true,
				secondsVisible: false
			}
		});

		rangeBars = chart.addCandlestickSeries({
			upColor: '#00CC00',
			downColor: '#CC0000',
			borderVisible: false,
			wickUpColor: '#00CC00',
			wickDownColor: '#CC0000',

			priceFormat: { type: 'price', precision: 5, minMove: 0.0002 }
		});

		const chartData = rangeData[0].map((item) => {
			return {
				time: item.start_time as Time, // Use the formatted time
				open: Number(item.open),
				high: Number(item.high),
				low: Number(item.low),
				close: Number(item.close)
			};
		});

		rangeBars.setData(chartData);

		chart.timeScale().fitContent();
	});
</script>

<div bind:this={chartContainer} />

<!-- <WatchedAssets symbols={watched_symbols} /> -->
