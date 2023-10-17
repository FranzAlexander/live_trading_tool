<script lang="ts">
	import type { OhlcEventPayload } from '$lib';
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

	let chart: IChartApi;
	let candlestickSeries: ISeriesApi<'Candlestick'>;
	let volumeSeries: ISeriesApi<'Histogram'>;
	let vwapSeries: ISeriesApi<'Line'>;

	let container: HTMLElement;

	listen('tauri://resize', (event) => {
		chart.resize(container.clientWidth, container.clientHeight);
	});

	// Listen to the `message-stream` event from the backend
	listen('message-stream', (event) => {
		console.log(event.payload);
		// Handle the streamed message as required
	});

	invoke('init_process');

	onMount(() => {
		const chartOptions: DeepPartial<ChartOptions> = {
			layout: { textColor: '#DDD', background: { color: '#000' } },
			grid: { vertLines: { color: '#444' }, horzLines: { color: '#444' } },
			rightPriceScale: {
				borderColor: '#71648C',
				scaleMargins: {
					top: 0.3,
					bottom: 0.25
				}
			},

			timeScale: { borderColor: '#71649C', timeVisible: true },

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
		};

		chart = createChart(container, chartOptions);

		volumeSeries = chart.addHistogramSeries({
			color: '#26a69a',
			baseLineWidth: 2,
			priceFormat: {
				type: 'volume',
				minMove: 0.002
			},
			priceScaleId: ''
		});

		vwapSeries = chart.addLineSeries({
			color: '#6699CC',
			lineWidth: 2,
			priceFormat: { type: 'volume', precision: 5, minMove: 0.002 }
		});

		candlestickSeries = chart.addCandlestickSeries({
			upColor: '#00CC00',
			downColor: '#CC0000',
			borderVisible: false,
			wickUpColor: '#00CC00',
			wickDownColor: '#CC0000',

			priceFormat: { type: 'price', precision: 5, minMove: 0.002 }
		});

		chart.priceScale('').applyOptions({
			scaleMargins: {
				top: 0.8,
				bottom: 0
			}
		});

		invoke('get_ohlc_history').then((result) => {
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
			// vwapSeries.setData(vwapData);
		});

		candlestickSeries.applyOptions({
			wickUpColor: '#00CC00',
			upColor: '#00CC00',
			wickDownColor: '#CC0000',
			downColor: '#CC0000',
			borderVisible: false,
			priceFormat: { type: 'price', precision: 5 }
		});
		chart.timeScale().fitContent();
	});
</script>

<main>
	<div bind:this={container} class="chart-container" />
</main>

<style>
	main {
		width: 100vw;
		height: 100vh;
		display: grid;
		grid-template-columns: repeat(4, minmax(0, 1fr));
		grid-template-rows: 10% repeat(3, minmax(0, 1fr));
	}

	.chart-container {
		grid-row: 2 / 4; /* Span from the 2nd to 3rd row */
		grid-column: 2 / 4; /* Span from the 2nd to 3rd column */
	}
</style>
