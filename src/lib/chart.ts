import {
	ColorType,
	createChart,
	CrosshairMode,
	LineStyle,
	type IChartApi
} from 'lightweight-charts';

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
		}
	});
}
