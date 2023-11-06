import {
	ColorType,
	createChart,
	CrosshairMode,
	LineStyle,
	type IChartApi,
	type DeepPartial,
	type PriceFormat,
	type CandlestickStyleOptions,
	type SeriesOptionsCommon
} from 'lightweight-charts';

// Define the proper structure for the priceFormat, assuming the type is 'price'
export const priceFormatConfig: DeepPartial<PriceFormat> = {
	type: 'price', // This should be a literal type 'price', 'volume', or 'percent'
	precision: 2,
	minMove: 0.01
};

// Now define the candlestickConfig with the proper typing
export const candlestickConfig: DeepPartial<CandlestickStyleOptions & SeriesOptionsCommon> = {
	upColor: '#00CC00',
	downColor: '#CC0000',
	borderVisible: false,
	wickUpColor: '#00CC00',
	wickDownColor: '#CC0000',
	priceFormat: priceFormatConfig // Use the defined priceFormatConfig here
};

export function createMainCandleChart(chartContainer: HTMLElement): IChartApi {
	return createChart(chartContainer, {
		layout: {
			background: {
				type: ColorType.Solid,
				color: '#000'
			},
			textColor: '#DDD'
		},
		grid: {
			vertLines: { color: '#444' },
			horzLines: { color: '#444' }
		},
		crosshair: {
			mode: CrosshairMode.Normal,
			vertLine: {
				color: '#C3BCDB44',
				style: LineStyle.Solid,
				width: 2,
				labelBackgroundColor: '#0000CC'
			},
			horzLine: {
				color: '#0000CC',
				labelBackgroundColor: '#0000CC'
			}
		},
		timeScale: {
			timeVisible: true,
			secondsVisible: true
		},
		rightPriceScale: { autoScale: true }
	});
}
