<script lang="ts">
	import type { OhlcEventPayload } from '$lib';
	import type { MacdHistory } from '$lib/indicators';
	import { listen } from '@tauri-apps/api/event';
	import {
		ColorType,
		createChart,
		CrosshairMode,
		LineStyle,
		type IChartApi,
		type ISeriesApi,
		type Time
	} from 'lightweight-charts';
	import { onMount } from 'svelte';

	export let initialOhlc: MacdHistory[];

	let macdChart: IChartApi;
	let macdHist: ISeriesApi<'Histogram'>;
	let macdLine: ISeriesApi<'Line'>;
	let macdSignal: ISeriesApi<'Line'>;

	let macdContainer: HTMLElement;
	listen('tauri://resize', (event) => {
		macdChart.resize(macdContainer.clientWidth, macdContainer.clientHeight);
	});
	onMount(async () => {
		macdChart = createChart(macdContainer, {
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

		macdHist = macdChart.addHistogramSeries({
			color: '#26a69a',
			baseLineWidth: 2,
			priceFormat: {
				type: 'volume',
				minMove: 0.00001
			},
			priceScaleId: ''
		});

		macdLine = macdChart.addLineSeries({
			color: '#fcd116',
			lineWidth: 2,

			priceFormat: { type: 'price', precision: 5, minMove: 0.00001 }
		});

		macdSignal = macdChart.addLineSeries({
			color: '#673ab7',
			lineWidth: 2,

			priceFormat: { type: 'price', precision: 5, minMove: 0.00001 }
		});

		macdChart.priceScale('').applyOptions({
			scaleMargins: {
				top: 0.8,
				bottom: 0
			}
		});

		const histogramData = initialOhlc.map((item) => {
			return {
				time: item.time as Time,
				value: Number(item.histogram),
				color: Number(item.histogram) > Number(0) ? '#26a69a' : '#e57373'
			};
		});

		// For the MACD Line
		const macdData = initialOhlc.map((item) => ({
			time: item.time as Time,
			value: Number(item.macd)
		}));

		// For the Signal Line
		const signalData = initialOhlc.map((item) => ({
			time: item.time as Time,
			value: Number(item.signal)
		}));

		// Set the data for each series
		macdHist.setData(histogramData);
		macdLine.setData(macdData);
		macdSignal.setData(signalData);

		// Fit the chart content
		macdChart.timeScale().fitContent();
	});
</script>

<div class="w-full h-64 flex-shrink" bind:this={macdContainer} />
